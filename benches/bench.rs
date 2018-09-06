#![feature(test)]
#![allow(unused_macros)]

extern crate packed_simd;
extern crate rand;
extern crate simd_prngs;
extern crate test;

use packed_simd::*;
use std::mem::*;

use test::Bencher;

use rand::prelude::*;
use simd_prngs::*;

const BENCH_N: u64 = 1 << 10;

/// Uniform float sampling for a more "real world" benchmark
/// This is not meant for serious use, check out https://crates.io/crates/rand
trait UniformFloat<T> {
    fn sample(self, scale: T, offset: T) -> T;
}

macro_rules! uniform_float_impl {
    ($fty:ident, $uty:ident, $u_scalar:ty, $bits_to_discard:expr, $fraction_bits:expr, $exponent_bias:expr) => {
        impl UniformFloat<$fty> for $uty {
            fn sample(self, scale: $fty, offset: $fty) -> $fty {
                #[inline(always)]
                fn into_float_with_exponent(x: $uty, exponent: i32) -> $fty {
                    // The exponent is encoded using an offset-binary representation
                    let exponent_bits: $u_scalar =
                        (($exponent_bias + exponent) as $u_scalar) << $fraction_bits;
                    $fty::from_bits(x | exponent_bits)
                }

                // Generate a value in the range [1, 2)
                let value: $uty = self >> $bits_to_discard;
                let value1_2 = into_float_with_exponent(value, 0);
                value1_2 * scale + offset
            }
        }
    };
}

uniform_float_impl! { f32x2, u32x2, u32, 32 - 23, 23, 127 }
uniform_float_impl! { f32x4, u32x4, u32, 32 - 23, 23, 127 }
uniform_float_impl! { f32x8, u32x8, u32, 32 - 23, 23, 127 }
uniform_float_impl! { f32x16, u32x16, u32, 32 - 23, 23, 127 }

uniform_float_impl! { f64x2, u64x2, u64, 64 - 52, 52, 1023 }
uniform_float_impl! { f64x4, u64x4, u64, 64 - 52, 52, 1023 }
uniform_float_impl! { f64x8, u64x8, u64, 64 - 52, 52, 1023 }

macro_rules! generate {
    ($fnn:ident, $gen:ident, $ty:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng = $gen::from_rng(thread_rng()).unwrap();
            b.iter(|| {
                let mut accum = $ty::default();
                for _ in 0..BENCH_N {
                    accum += rng.generate();
                }
                accum
            });
            b.bytes = BENCH_N * size_of::<$ty>() as u64;
        }
    };
}

macro_rules! float {
    ($fnn:ident, $gen:ident, $fty:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng: $gen = $gen::from_rng(thread_rng()).unwrap();

            let low = $fty::splat(2.26);
            let high = $fty::splat(2.319);

            let scale = high - low;
            let offset = low - scale;

            b.iter(|| {
                let mut accum = $fty::default();
                for _ in 0..BENCH_N {
                    accum += rng.generate().sample(scale, offset);;
                }
                accum
            });
            b.bytes = BENCH_N * size_of::<$fty>() as u64;
        }
    };
}

// uniform integer bounding using a widening multiply method
macro_rules! wmul_int {
    ($fnn:ident, $gen:ident, $uty:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            const BITS: usize = size_of::<$uty>() * 8 / $uty::lanes();

            let mut rng: $gen = $gen::from_rng(thread_rng()).unwrap();

            let low = $uty::splat(0);
            let high = $uty::splat((1 << (BITS - 1)) + 1);

            let unsigned_max = !0;
            let range: $uty = (high - low).cast();
            let ints_to_reject = (unsigned_max - range + 1) % range;
            let zone = unsigned_max - ints_to_reject;

            b.iter(|| {
                let mut accum = $uty::default();
                for _ in 0..BENCH_N {
                    let mut v: $uty = $uty::from_bits(rng.generate());
                    let mut hi = $uty::default();
                    for _ in 0..$uty::lanes().trailing_zeros() + 1 {
                        let (hi_2, lo) = v.wmul(range);
                        hi = hi_2;
                        let cmp = lo.le(zone);
                        // explicit `movmsk`: https://github.com/rust-lang-nursery/packed_simd/issues/103
                        let int_mask = unsafe { _mm_movemask_epi8(__m128i::from_bits(cmp)) };
                        test::black_box(int_mask == u8::max_value() as i32);
                        v = cmp.select(v, $uty::from_bits(rng.generate()));
                    }
                    accum += low + hi;
                }
                accum
            });
            b.bytes = BENCH_N * size_of::<$uty>() as u64;
        }
    };
}

// uniform integer bounding using a bitmask method
macro_rules! bitmask_int {
    ($fnn:ident, $gen:ident, $uty:ident, $u_scalar:ty) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            const BITS: usize = size_of::<$uty>() * 8 / $uty::lanes();

            let mut rng: $gen = $gen::from_rng(thread_rng()).unwrap();

            let low: $u_scalar = 0;
            let high = (1 << (BITS - 1)) + 1;

            let mut range = high - low;
            range -= 1;
            let zeros = (range | 1).leading_zeros();
            let mask = $uty::splat(!0 >> zeros);

            b.iter(|| {
                let mut accum = $uty::default();
                for _ in 0..BENCH_N {
                    let mut x = $uty::from_bits(rng.generate()) & mask;
                    // reject x > range
                    for _ in 0..$uty::lanes().trailing_zeros() + 1 {
                        let cmp = x.le($uty::splat(range));
                        // explicit `movmsk`: https://github.com/rust-lang-nursery/packed_simd/issues/103
                        let int_mask = unsafe { _mm_movemask_epi8(__m128i::from_bits(cmp)) };
                        test::black_box(int_mask == u8::max_value() as i32);
                        x = cmp.select(x, $uty::from_bits(rng.generate()) & mask);
                    }
                }
                accum
            });
            b.bytes = BENCH_N * size_of::<$uty>() as u64;
        }
    };
}

// uniform integer bounding, for use-case benchmarking
macro_rules! int {
    ($wmul:ident, $bitmask:ident, $gen:ident, $uty:ident, $u_scalar:ty) => {
        wmul_int!($wmul, $gen, $uty);
        bitmask_int!($bitmask, $gen, $uty, $u_scalar);
    };
}

// benchmark PRNG initialization
macro_rules! init {
    ($fnn:ident, $gen:ident, $init:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng = rand::XorShiftRng::from_rng(thread_rng()).unwrap();
            b.iter(|| {
                let r2 = $gen::$init(&mut rng).unwrap();
                r2
            });
        }
    };
}

// general benchmarking macro
macro_rules! bench {
    ($generate:ident, $float:ident, $gen:ident, $uty:ident, $fty:ident) => {
        generate! { $generate, $gen, $uty }
        float! { $float, $gen, $fty }
    };
}

bench! { gen_ars5, float_ars5, Ars5, u64x2, f64x2 }
bench! { gen_ars7, float_ars7, Ars7, u64x2, f64x2 }

bench! { gen_sfc64_x2, float_sfc64_x2, Sfc64x2, u64x2, f64x2 }
bench! { gen_sfc64_x4, float_sfc64_x4, Sfc64x4, u64x4, f64x4 }
bench! { gen_sfc64_x8, float_sfc64_x8, Sfc64x8, u64x8, f64x8 }

bench! { gen_sfc32_x2, float_sfc32_x2, Sfc32x2, u32x2, f32x2 }
bench! { gen_sfc32_x4, float_sfc32_x4, Sfc32x4, u32x4, f32x4 }
bench! { gen_sfc32_x8, float_sfc32_x8, Sfc32x8, u32x8, f32x8 }
bench! { gen_sfc32_x16, float_sfc32_x16, Sfc32x16, u32x16, f32x16 }

generate! { gen_sfc16_x2, Sfc16x2, u16x2 }
generate! { gen_sfc16_x4, Sfc16x4, u16x4 }
generate! { gen_sfc16_x8, Sfc16x8, u16x8 }
generate! { gen_sfc16_x16, Sfc16x16, u16x16 }
generate! { gen_sfc16_x32, Sfc16x32, u16x32 }

bench! { gen_jsf32_x2, float_jsf32_x2, Jsf32x2, u32x2, f32x2 }
bench! { gen_jsf32_x4, float_jsf32_x4, Jsf32x4, u32x4, f32x4 }
bench! { gen_jsf32_x8, float_jsf32_x8, Jsf32x8, u32x8, f32x8 }
bench! { gen_jsf32_x16, float_jsf32_x16, Jsf32x16, u32x16, f32x16 }

bench! { gen_jsf64_x2, float_jsf64_x2, Jsf64x2, u64x2, f64x2 }
bench! { gen_jsf64_x4, float_jsf64_x4, Jsf64x4, u64x4, f64x4 }
bench! { gen_jsf64_x8, float_jsf64_x8, Jsf64x8, u64x8, f64x8 }

bench! { gen_xorshift32_x2, float_xorshift32_x2, Xorshift32x2, u32x2, f32x2 }
bench! { gen_xorshift32_x4, float_xorshift32_x4, Xorshift32x4, u32x4, f32x4 }
bench! { gen_xorshift32_x8, float_xorshift32_x8, Xorshift32x8, u32x8, f32x8 }
bench! { gen_xorshift32_x16, float_xorshift32_x16, Xorshift32x16, u32x16, f32x16 }

bench! { gen_xorshift128_x2, float_xorshift128_x2, Xorshift128x2, u32x2, f32x2 }
bench! { gen_xorshift128_x4, float_xorshift128_x4, Xorshift128x4, u32x4, f32x4 }
bench! { gen_xorshift128_x8, float_xorshift128_x8, Xorshift128x8, u32x8, f32x8 }
bench! { gen_xorshift128_x16, float_xorshift128_x16, Xorshift128x16, u32x16, f32x16 }

bench! { gen_xorshift128plus_x2, float_xorshift128plus_x2, Xorshift128PlusX2, u64x2, f64x2 }
bench! { gen_xorshift128plus_x4, float_xorshift128plus_x4, Xorshift128PlusX4, u64x4, f64x4 }
bench! { gen_xorshift128plus_x8, float_xorshift128plus_x8, Xorshift128PlusX8, u64x8, f64x8 }

bench! { gen_xoroshiro128starstar_x2, float_xoroshiro128starstar_x2, Xoroshiro128StarStarX2, u64x2, f64x2 }
bench! { gen_xoroshiro128starstar_x4, float_xoroshiro128starstar_x4, Xoroshiro128StarStarX4, u64x4, f64x4 }
bench! { gen_xoroshiro128starstar_x8, float_xoroshiro128starstar_x8, Xoroshiro128StarStarX8, u64x8, f64x8 }

init! { init_jumps_xoroshiro128starstar_x2, Xoroshiro128StarStarX2, blocks_from_rng }
init! { init_jumps_xoroshiro128starstar_x4, Xoroshiro128StarStarX4, blocks_from_rng }
init! { init_jumps_xoroshiro128starstar_x8, Xoroshiro128StarStarX8, blocks_from_rng }

init! { init_rand_xoroshiro128starstar_x4, Xoroshiro128StarStarX4, from_rng }
init! { init_rand_xoroshiro128starstar_x2, Xoroshiro128StarStarX2, from_rng }
init! { init_rand_xoroshiro128starstar_x8, Xoroshiro128StarStarX8, from_rng }

bench! { gen_xoshiro256starstar_x2, float_xoshiro256starstar_x2, Xoshiro256StarStarX2, u64x2, f64x2 }
bench! { gen_xoshiro256starstar_x4, float_xoshiro256starstar_x4, Xoshiro256StarStarX4, u64x4, f64x4 }
bench! { gen_xoshiro256starstar_x8, float_xoshiro256starstar_x8, Xoshiro256StarStarX8, u64x8, f64x8 }

init! { init_jumps_xoshiro256starstar_x2, Xoshiro256StarStarX2, blocks_from_rng }
init! { init_jumps_xoshiro256starstar_x4, Xoshiro256StarStarX4, blocks_from_rng }
init! { init_jumps_xoshiro256starstar_x8, Xoshiro256StarStarX8, blocks_from_rng }

init! { init_rand_xoshiro256starstar_x2, Xoshiro256StarStarX2, from_rng }
init! { init_rand_xoshiro256starstar_x4, Xoshiro256StarStarX4, from_rng }
init! { init_rand_xoshiro256starstar_x8, Xoshiro256StarStarX8, from_rng }

bench! { gen_xoshiro512starstar_x2, float_xoshiro512starstar_x2, Xoshiro512StarStarX2, u64x2, f64x2 }
bench! { gen_xoshiro512starstar_x4, float_xoshiro512starstar_x4, Xoshiro512StarStarX4, u64x4, f64x4 }
bench! { gen_xoshiro512starstar_x8, float_xoshiro512starstar_x8, Xoshiro512StarStarX8, u64x8, f64x8 }

bench! { gen_lcg32x2, float_lcg32x2, Lcg32x2, u32x2, f32x2 }
bench! { gen_lcg32x4, float_lcg32x4, Lcg32x4, u32x4, f32x4 }
bench! { gen_lcg32x8, float_lcg32x8, Lcg32x8, u32x8, f32x8 }

generate! { gen_lcg16x2, Lcg16x2, u16x2 }
generate! { gen_lcg16x4, Lcg16x4, u16x4 }
generate! { gen_lcg16x8, Lcg16x8, u16x8 }
generate! { gen_lcg16x16, Lcg16x8, u16x8 }

bench! { gen_pcg32x2, float_pcg32x2, Pcg32x2, u32x2, f32x2 }
bench! { gen_pcg32x4, float_pcg32x4, Pcg32x4, u32x4, f32x4 }
bench! { gen_pcg32x8, float_pcg32x8, Pcg32x8, u32x8, f32x8 }

bench! { gen_pcg_fixed_xsh32x2, float_pcg_fixed_xsh32x2, PcgFixedXsh32x2, u32x2, f32x2 }
bench! { gen_pcg_fixed_xsh32x4, float_pcg_fixed_xsh32x4, PcgFixedXsh32x4, u32x4, f32x4 }
bench! { gen_pcg_fixed_xsh32x8, float_pcg_fixed_xsh32x8, PcgFixedXsh32x8, u32x8, f32x8 }

bench! { gen_pcg_fixed_xsl32x2, float_pcg_fixed_xsl32x2, PcgFixedXsl32x2, u32x2, f32x2 }
bench! { gen_pcg_fixed_xsl32x4, float_pcg_fixed_xsl32x4, PcgFixedXsl32x4, u32x4, f32x4 }
bench! { gen_pcg_fixed_xsl32x8, float_pcg_fixed_xsl32x8, PcgFixedXsl32x8, u32x8, f32x8 }

bench! { gen_lfsr113_x2, float_lfsr113_x2, Lfsr113x2, u32x2, f32x2 }
bench! { gen_lfsr113_x4, float_lfsr113_x4, Lfsr113x4, u32x4, f32x4 }
bench! { gen_lfsr113_x8, float_lfsr113_x8, Lfsr113x8, u32x8, f32x8 }
bench! { gen_lfsr113_x16, float_lfsr113_x16, Lfsr113x16, u32x16, f32x16 }

bench! { gen_lfsr258_x2, float_lfsr258_x2, Lfsr258x2, u64x2, f64x2 }
bench! { gen_lfsr258_x4, float_lfsr258_x4, Lfsr258x4, u64x4, f64x4 }
bench! { gen_lfsr258_x8, float_lfsr258_x8, Lfsr258x8, u64x8, f64x8 }

bench! { gen_mwc8, float_mwc8, Mwc8, u64x2, f64x2 }
bench! { gen_mwc4, float_mwc4, Mwc4, u64x2, f64x2 }
bench! { gen_mwc2, float_mwc2, Mwc2, u64x2, f64x2 }

bench! { gen_xsm64_x2, float_xsm64_x2, Xsm64x2, u64x2, f64x2 }
bench! { gen_xsm64_x4, float_xsm64_x4, Xsm64x4, u64x4, f64x4 }
bench! { gen_xsm64_x8, float_xsm64_x8, Xsm64x8, u64x8, f64x8 }

bench! { gen_xsm32_x2, float_xsm32_x2, Xsm32x2, u32x2, f32x2 }
bench! { gen_xsm32_x4, float_xsm32_x4, Xsm32x4, u32x4, f32x4 }
bench! { gen_xsm32_x8, float_xsm32_x8, Xsm32x8, u32x8, f32x8 }
bench! { gen_xsm32_x16, float_xsm32_x16, Xsm32x16, u32x16, f32x16 }

init! { init_block_xsm32_x2, Xsm32x2, blocks_from_rng }
init! { init_rand_xsm32_x2, Xsm32x2, from_rng }
init! { init_block_xsm32_x4, Xsm32x4, blocks_from_rng }
init! { init_rand_xsm32_x4, Xsm32x4, from_rng }
init! { init_block_xsm32_x8, Xsm32x8, blocks_from_rng }
init! { init_rand_xsm32_x8, Xsm32x8, from_rng }
init! { init_block_xsm32_x16, Xsm32x16, blocks_from_rng }
init! { init_rand_xsm32_x16, Xsm32x16, from_rng }

bench! { gen_intel_lcg, float_intel_lcg, IntelLcg, u32x4, f32x4 }

bench! { gen_chacha4, float_chacha4, ChaCha4, u32x16, f32x16 }
bench! { gen_chacha4_a, float_chacha4_a, ChaChaA4, u32x16, f32x16 }
