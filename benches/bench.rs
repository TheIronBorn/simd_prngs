#![feature(stdsimd)]
#![feature(test)]

extern crate rand;
extern crate simd_prngs;
extern crate test;

use std::mem::*;
use std::simd::*;

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

macro_rules! bench {
    ($generate:ident, $float:ident, $rng:ident, $uty:ident, $fty:ident) => {
        #[bench]
        fn $generate(b: &mut Bencher) {
            let mut rng: $rng = $rng::from_rng(thread_rng()).unwrap();
            b.iter(|| {
                let mut accum = $uty::default();
                for _ in 0..BENCH_N {
                    accum += rng.generate();
                }
                accum
            });
            b.bytes = BENCH_N * size_of::<$uty>() as u64;
        }

        #[bench]
        fn $float(b: &mut Bencher) {
            let mut rng: $rng = $rng::from_rng(thread_rng()).unwrap();

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

bench! { ars5, f_ars5, Ars5, u64x2, f64x2 }
bench! { ars7, f_ars7, Ars7, u64x2, f64x2 }

bench! { sfc64_x2, f_sfc64_x2, Sfc64x2, u64x2, f64x2 }
bench! { sfc64_x4, f_sfc64_x4, Sfc64x4, u64x4, f64x4 }
bench! { sfc64_x8, f_sfc64_x8, Sfc64x8, u64x8, f64x8 }

bench! { sfc32_x2, f_sfc32_x2, Sfc32x2, u32x2, f32x2 }
bench! { sfc32_x4, f_sfc32_x4, Sfc32x4, u32x4, f32x4 }
bench! { sfc32_x8, f_sfc32_x8, Sfc32x8, u32x8, f32x8 }
bench! { sfc32_x16, f_sfc32_x16, Sfc32x16, u32x16, f32x16 }

/*bench! { sfc16_x2, f_sfc16_x2, Sfc16x2, u16x2, f16x2 }
bench! { sfc16_x4, f_sfc16_x4, Sfc16x4, u16x4, f16x4 }
bench! { sfc16_x8, f_sfc16_x8, Sfc16x8, u16x8, f16x8 }
bench! { sfc16_x16, f_sfc16_x16, Sfc16x16, u16x16, f16x16 }
bench! { sfc16_x32, f_sfc16_x32, Sfc16x32, u16x32, f16x32 }*/

bench! { jsf32_x2, f_jsf32_x2, Jsf32x2, u32x2, f32x2 }
bench! { jsf32_x4, f_jsf32_x4, Jsf32x4, u32x4, f32x4 }
bench! { jsf32_x8, f_jsf32_x8, Jsf32x8, u32x8, f32x8 }
bench! { jsf32_x16, f_jsf32_x16, Jsf32x16, u32x16, f32x16 }

bench! { jsf64_x2, f_jsf64_x2, Jsf64x2, u64x2, f64x2 }
bench! { jsf64_x4, f_jsf64_x4, Jsf64x4, u64x4, f64x4 }
bench! { jsf64_x8, f_jsf64_x8, Jsf64x8, u64x8, f64x8 }

bench! { xorshift32_x2, f_xorshift32_x2, Xorshift32x2, u32x2, f32x2 }
bench! { xorshift32_x4, f_xorshift32_x4, Xorshift32x4, u32x4, f32x4 }
bench! { xorshift32_x8, f_xorshift32_x8, Xorshift32x8, u32x8, f32x8 }
bench! { xorshift32_x16, f_xorshift32_x16, Xorshift32x16, u32x16, f32x16 }

bench! { xorshift128_x2, f_xorshift128_x2, Xorshift128x2, u32x2, f32x2 }
bench! { xorshift128_x4, f_xorshift128_x4, Xorshift128x4, u32x4, f32x4 }
bench! { xorshift128_x8, f_xorshift128_x8, Xorshift128x8, u32x8, f32x8 }
bench! { xorshift128_x16, f_xorshift128_x16, Xorshift128x16, u32x16, f32x16 }

bench! { xorshift128plus_x2, f_xorshift128plus_x2, Xorshift128PlusX2, u64x2, f64x2 }
bench! { xorshift128plus_x4, f_xorshift128plus_x4, Xorshift128PlusX4, u64x4, f64x4 }
bench! { xorshift128plus_x8, f_xorshift128plus_x8, Xorshift128PlusX8, u64x8, f64x8 }

bench! { xoroshiro128starstar_x2, f_xoroshiro128starstar_x2, Xoroshiro128StarStarX2, u64x2, f64x2 }
bench! { xoroshiro128starstar_x4, f_xoroshiro128starstar_x4, Xoroshiro128StarStarX4, u64x4, f64x4 }
bench! { xoroshiro128starstar_x8, f_xoroshiro128starstar_x8, Xoroshiro128StarStarX8, u64x8, f64x8 }

bench! { xoshiro256starstar_x2, f_xoshiro256starstar_x2, Xoshiro256StarStarX2, u64x2, f64x2 }
bench! { xoshiro256starstar_x4, f_xoshiro256starstar_x4, Xoshiro256StarStarX4, u64x4, f64x4 }
bench! { xoshiro256starstar_x8, f_xoshiro256starstar_x8, Xoshiro256StarStarX8, u64x8, f64x8 }

bench! { lcg32x2, f_lcg32x2, Lcg32x2, u32x2, f32x2 }
bench! { lcg32x4, f_lcg32x4, Lcg32x4, u32x4, f32x4 }
bench! { lcg32x8, f_lcg32x8, Lcg32x8, u32x8, f32x8 }

bench! { pcg32x2, f_pcg32x2, Pcg32x2, u32x2, f32x2 }
bench! { pcg32x4, f_pcg32x4, Pcg32x4, u32x4, f32x4 }
bench! { pcg32x8, f_pcg32x8, Pcg32x8, u32x8, f32x8 }

bench! { pcg_fixed_xsh32x2, f_pcg_fixed_xsh32x2, PcgFixedXsh32x2, u32x2, f32x2 }
bench! { pcg_fixed_xsh32x4, f_pcg_fixed_xsh32x4, PcgFixedXsh32x4, u32x4, f32x4 }
bench! { pcg_fixed_xsh32x8, f_pcg_fixed_xsh32x8, PcgFixedXsh32x8, u32x8, f32x8 }

bench! { pcg_fixed_xsl32x2, f_pcg_fixed_xsl32x2, PcgFixedXsl32x2, u32x2, f32x2 }
bench! { pcg_fixed_xsl32x4, f_pcg_fixed_xsl32x4, PcgFixedXsl32x4, u32x4, f32x4 }
bench! { pcg_fixed_xsl32x8, f_pcg_fixed_xsl32x8, PcgFixedXsl32x8, u32x8, f32x8 }

bench! { lfsr113_x2, f_lfsr113_x2, Lfsr113x2, u32x2, f32x2 }
bench! { lfsr113_x4, f_lfsr113_x4, Lfsr113x4, u32x4, f32x4 }
bench! { lfsr113_x8, f_lfsr113_x8, Lfsr113x8, u32x8, f32x8 }
bench! { lfsr113_x16, f_lfsr113_x16, Lfsr113x16, u32x16, f32x16 }

bench! { lfsr258_x2, f_lfsr258_x2, Lfsr258x2, u64x2, f64x2 }
bench! { lfsr258_x4, f_lfsr258_x4, Lfsr258x4, u64x4, f64x4 }
bench! { lfsr258_x8, f_lfsr258_x8, Lfsr258x8, u64x8, f64x8 }

bench! { mwc8, f_mwc8, Mwc8, u64x2, f64x2 }
bench! { mwc4, f_mwc4, Mwc4, u64x2, f64x2 }
bench! { mwc2, f_mwc2, Mwc2, u64x2, f64x2 }

bench! { xsm64_x2, f_xsm64_x2, Xsm64x2, u64x2, f64x2 }
bench! { xsm64_x4, f_xsm64_x4, Xsm64x4, u64x4, f64x4 }
bench! { xsm64_x8, f_xsm64_x8, Xsm64x8, u64x8, f64x8 }

bench! { xsm32_x2, f_xsm32_x2, Xsm32x2, u32x2, f32x2 }
bench! { xsm32_x4, f_xsm32_x4, Xsm32x4, u32x4, f32x4 }
bench! { xsm32_x8, f_xsm32_x8, Xsm32x8, u32x8, f32x8 }
bench! { xsm32_x16, f_xsm32_x16, Xsm32x16, u32x16, f32x16 }

bench! { intel_lcg, f_intel_lcg, IntelLcg, u32x4, f32x4 }
