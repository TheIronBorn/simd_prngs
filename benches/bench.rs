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

macro_rules! generate {
    ($fnn:ident, $rng:ident, $vec:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng: $rng = $rng::from_rng(thread_rng()).unwrap();
            b.iter(|| {
                let mut accum = $vec::default();
                for _ in 0..BENCH_N {
                    accum += rng.generate();
                }
                accum
            });
            b.bytes = BENCH_N * size_of::<$vec>() as u64;
        }
    };
}

generate! { ars5, Ars5, u64x2 }
generate! { ars7, Ars7, u64x2 }

generate! { sfc64_x2, Sfc64x2, u64x2 }
generate! { sfc64_x4, Sfc64x4, u64x4 }
generate! { sfc64_x8, Sfc64x8, u64x8 }

generate! { sfc32_x2, Sfc32x2, u32x2 }
generate! { sfc32_x4, Sfc32x4, u32x4 }
generate! { sfc32_x8, Sfc32x8, u32x8 }
generate! { sfc32_x16, Sfc32x16, u32x16 }

generate! { sfc16_x2, Sfc16x2, u16x2 }
generate! { sfc16_x4, Sfc16x4, u16x4 }
generate! { sfc16_x8, Sfc16x8, u16x8 }
generate! { sfc16_x16, Sfc16x16, u16x16 }
generate! { sfc16_x32, Sfc16x32, u16x32 }

generate! { jsf32_x2, Jsf32x2, u32x2 }
generate! { jsf32_x4, Jsf32x4, u32x4 }
generate! { jsf32_x8, Jsf32x8, u32x8 }
generate! { jsf32_x16, Jsf32x16, u32x16 }

generate! { jsf64_x2, Jsf64x2, u64x2 }
generate! { jsf64_x4, Jsf64x4, u64x4 }
generate! { jsf64_x8, Jsf64x8, u64x8 }

generate! { xorshift32_x2, Xorshift32x2, u32x2 }
generate! { xorshift32_x4, Xorshift32x4, u32x4 }
generate! { xorshift32_x8, Xorshift32x8, u32x8 }
generate! { xorshift32_x16, Xorshift32x16, u32x16 }

generate! { xorshift128_x2, Xorshift128x2, u32x2 }
generate! { xorshift128_x4, Xorshift128x4, u32x4 }
generate! { xorshift128_x8, Xorshift128x8, u32x8 }
generate! { xorshift128_x16, Xorshift128x16, u32x16 }

generate! { xorshift128plus_x2, Xorshift128PlusX2, u64x2 }
generate! { xorshift128plus_x4, Xorshift128PlusX4, u64x4 }
generate! { xorshift128plus_x8, Xorshift128PlusX8, u64x8 }

generate! { xoroshiro128starstar_x2, Xoroshiro128StarStarX2, u64x2 }
generate! { xoroshiro128starstar_x4, Xoroshiro128StarStarX4, u64x4 }
generate! { xoroshiro128starstar_x8, Xoroshiro128StarStarX8, u64x8 }

generate! { xoshiro256starstar_x2, Xoshiro256StarStarX2, u64x2 }
generate! { xoshiro256starstar_x4, Xoshiro256StarStarX4, u64x4 }
generate! { xoshiro256starstar_x8, Xoshiro256StarStarX8, u64x8 }

generate! { lcg32x2, Lcg32x2, u32x2 }
generate! { lcg32x4, Lcg32x4, u32x4 }
generate! { lcg32x8, Lcg32x8, u32x8 }

generate! { pcg32x2, Pcg32x2, u32x2 }
generate! { pcg32x4, Pcg32x4, u32x4 }
generate! { pcg32x8, Pcg32x8, u32x8 }

generate! { pcg_fixed_xsh32x2, PcgFixedXsh32x2, u32x2 }
generate! { pcg_fixed_xsh32x4, PcgFixedXsh32x4, u32x4 }
generate! { pcg_fixed_xsh32x8, PcgFixedXsh32x8, u32x8 }

generate! { pcg_fixed_xsl32x2, PcgFixedXsl32x2, u32x2 }
generate! { pcg_fixed_xsl32x4, PcgFixedXsl32x4, u32x4 }
generate! { pcg_fixed_xsl32x8, PcgFixedXsl32x8, u32x8 }

generate! { lfsr113_x2, Lfsr113x2, u32x2 }
generate! { lfsr113_x4, Lfsr113x4, u32x4 }
generate! { lfsr113_x8, Lfsr113x8, u32x8 }
generate! { lfsr113_x16, Lfsr113x16, u32x16 }

generate! { lfsr258_x2, Lfsr258x2, u64x2 }
generate! { lfsr258_x4, Lfsr258x4, u64x4 }
generate! { lfsr258_x8, Lfsr258x8, u64x8 }

generate! { mwc8, Mwc8, u64x2 }
generate! { mwc4, Mwc4, u64x2 }
generate! { mwc2, Mwc2, u64x2 }

generate! { xsm64_x2, Xsm64x2, u64x2 }
generate! { xsm64_x4, Xsm64x4, u64x4 }
generate! { xsm64_x8, Xsm64x8, u64x8 }

generate! { xsm32_x2, Xsm32x2, u32x2 }
generate! { xsm32_x4, Xsm32x4, u32x4 }
generate! { xsm32_x8, Xsm32x8, u32x8 }
generate! { xsm32_x16, Xsm32x16, u32x16 }

generate! { intel_lcg, IntelLcg, u32x4 }
