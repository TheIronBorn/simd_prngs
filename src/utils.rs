/// Builds the shuffle indices for `RotateOpts`.
#[allow(dead_code)]
fn build_indices() {
    fn build_left(bytes: usize, lane_bytes: usize, rot: usize) {
        let mut arr: Vec<_> = (0..bytes).collect();
        for chunk in arr.chunks_exact_mut(lane_bytes) {
            chunk.rotate_left(rot);
        }
        println!("{}, {:?}", rot * 8, arr);
    }

    // 16-bit
    for i in 2..7 {
        let bytes = 1 << i;
        for rot in 1..2 {
            build_left(bytes, 2, rot);
        }
    }

    println!();

    // 32-bit
    for i in 3..7 {
        let bytes = 1 << i;
        for rot in 1..4 {
            build_left(bytes, 4, rot);
        }
    }

    println!();

    // 64-bit
    for i in 4..7 {
        let bytes = 1 << i;
        for rot in 1..8 {
            build_left(bytes, 8, rot);
        }
    }
}

macro_rules! impl_rngcore {
    ($rng:ident) => {
        impl RngCore for $rng {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                self.generate_u32()
            }

            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                self.generate_u64()
            }

            #[inline(always)]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                self.fill_bytes_unaligned(dest)
            }

            #[inline(always)]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
                self.fill_bytes(dest);
                Ok(())
            }
        }
    };
}

// exported for use in external benchmarks
#[doc(hidden)]
#[macro_export]
macro_rules! for_each_prng {
    ($macro:ident) => {
        $macro! { AesRand, u32x4, f32x4 }

        $macro! { Ars5, u32x4, f32x4 }
        $macro! { Ars7, u32x4, f32x4 }

        $macro! { ChaCha4, u32x4, f32x4 }
        $macro! { ChaChaAlt4, u32x4, f32x4 }

        $macro! { IntelLcg, u32x4, f32x4 }

        $macro! { Jsf32x2, u32x2, f32x2 }
        $macro! { Jsf32x4, u32x4, f32x4 }
        $macro! { Jsf32x8, u32x8, f32x8 }
        $macro! { Jsf32x16, u32x16, f32x16 }

        $macro! { Jsf64x2, u32x2, f32x2 }
        $macro! { Jsf64x4, u32x4, f32x4 }
        $macro! { Jsf64x8, u32x8, f32x8 }

        $macro! { Lcg16x2, u16x2, f32x2 } // too small for SIMD floats
        $macro! { Lcg16x4, u16x4, f32x2 }
        $macro! { Lcg16x8, u16x8, f32x4 }
        $macro! { Lcg16x16, u16x16, f32x8 }

        $macro! { Lcg32x2, u32x2, f32x2 }
        $macro! { Lcg32x4, u32x4, f32x4 }
        $macro! { Lcg32x8, u32x8, f32x8 }

        $macro! { Lfsr113x2, u32x2, f32x2 }
        $macro! { Lfsr113x4, u32x4, f32x4 }
        $macro! { Lfsr113x8, u32x8, f32x8 }
        $macro! { Lfsr113x16, u32x16, f32x16 }

        $macro! { Lfsr258x2, u32x2, f32x2 }
        $macro! { Lfsr258x4, u32x4, f32x4 }
        $macro! { Lfsr258x8, u32x8, f32x8 }

        $macro! { Mwc2, u32x2, f32x2 }
        $macro! { Mwc4, u32x4, f32x4 }
        $macro! { Mwc8, u32x8, f32x8 }

        $macro! { Pcg32x2, u32x2, f32x2 }
        $macro! { Pcg32x4, u32x4, f32x4 }
        $macro! { Pcg32x8, u32x8, f32x8 }

        $macro! { PcgFixedXsh32x2, u32x2, f32x2 }
        $macro! { PcgFixedXsh32x4, u32x4, f32x4 }
        $macro! { PcgFixedXsh32x8, u32x8, f32x8 }

        $macro! { PcgFixedXsl32x2, u32x2, f32x2 }
        $macro! { PcgFixedXsl32x4, u32x4, f32x4 }
        $macro! { PcgFixedXsl32x8, u32x8, f32x8 }

        $macro! { Sfc16x2, u16x2, f32x2 } // too small for SIMD floats
        $macro! { Sfc16x4, u16x4, f32x2 }
        $macro! { Sfc16x8, u16x8, f32x4 }
        $macro! { Sfc16x16, u16x16, f32x8 }
        $macro! { Sfc16x32, u16x32, f32x16 }

        $macro! { Sfc32x2, u32x2, f32x2 }
        $macro! { Sfc32x4, u32x4, f32x4 }
        $macro! { Sfc32x8, u32x8, f32x8 }
        $macro! { Sfc32x16, u32x16, f32x16 }

        $macro! { Sfc64x2, u32x2, f32x2 }
        $macro! { Sfc64x4, u32x4, f32x4 }
        $macro! { Sfc64x8, u32x8, f32x8 }

        $macro! { Xoroshiro128StarStarX2, u64x2, f32x4 }
        $macro! { Xoroshiro128StarStarX4, u64x4, f32x8 }
        $macro! { Xoroshiro128StarStarX8, u64x8, f32x16 }

        $macro! { Xorshift32x16, u32x2, f32x2 }
        $macro! { Xorshift32x2, u32x4, f32x4 }
        $macro! { Xorshift32x4, u32x8, f32x8 }
        $macro! { Xorshift32x8, u32x16, f32x16 }

        $macro! { Xorshift128x2, u32x2, f32x2 }
        $macro! { Xorshift128x4, u32x4, f32x4 }
        $macro! { Xorshift128x8, u32x8, f32x8 }
        $macro! { Xorshift128x16, u32x16, f32x16 }

        $macro! { Xorshift128PlusX2, u32x2, f32x2 }
        $macro! { Xorshift128PlusX4, u32x4, f32x4 }
        $macro! { Xorshift128PlusX8, u32x8, f32x8 }

        $macro! { Xoshiro128StarStarX2, u32x2, f32x2 }
        $macro! { Xoshiro128StarStarX4, u32x4, f32x4 }
        $macro! { Xoshiro128StarStarX8, u32x8, f32x8 }
        $macro! { Xoshiro128StarStarX16, u32x16, f32x16 }

        $macro! { Xoshiro256StarStarX2, u32x2, f32x2 }
        $macro! { Xoshiro256StarStarX4, u32x4, f32x4 }
        $macro! { Xoshiro256StarStarX8, u32x8, f32x8 }

        $macro! { Xoshiro512StarStarX2, u32x2, f32x2 }
        $macro! { Xoshiro512StarStarX4, u32x4, f32x4 }
        $macro! { Xoshiro512StarStarX8, u32x8, f32x8 }

        $macro! { Xsm32x2, u32x2, f32x2 }
        $macro! { Xsm32x4, u32x4, f32x4 }
        $macro! { Xsm32x8, u32x8, f32x8 }
        $macro! { Xsm32x16, u32x16, f32x16 }

        $macro! { Xsm64x2, u32x2, f32x2 }
        $macro! { Xsm64x4, u32x4, f32x4 }
        $macro! { Xsm64x8, u32x8, f32x8 }

        #[cfg(feature = "candidate_rngs")]
        mod candidate_rngs {
            use super::*;

            $macro! { SfcAlt64x2a, u64x2, f64x2 }
            $macro! { SfcAlt64x2b, u64x2, f64x2 }
            $macro! { SfcAlt64x2c, u64x2, f64x2 }
            $macro! { SfcAlt64x2d, u64x2, f64x2 }
            $macro! { SfcAlt64x2e, u64x2, f64x2 }
            $macro! { SfcAlt64x2f, u64x2, f64x2 }
            $macro! { SfcAlt64x2g, u64x2, f64x2 }
            $macro! { SfcAlt64x2h, u64x2, f64x2 }
            $macro! { SfcAlt64x2i, u64x2, f64x2 }
            $macro! { SfcAlt64x2j, u64x2, f64x2 }
            $macro! { SfcAlt64x2k, u64x2, f64x2 }
            $macro! { SfcAlt64x2l, u64x2, f64x2 }
            $macro! { SfcAlt64x4a, u64x4, f64x4 }
            $macro! { SfcAlt64x4b, u64x4, f64x4 }
            $macro! { SfcAlt64x4c, u64x4, f64x4 }
            $macro! { SfcAlt64x4d, u64x4, f64x4 }
            $macro! { SfcAlt64x4e, u64x4, f64x4 }
            $macro! { SfcAlt64x4f, u64x4, f64x4 }
            $macro! { SfcAlt64x4g, u64x4, f64x4 }
            $macro! { SfcAlt64x4h, u64x4, f64x4 }
            $macro! { SfcAlt64x4i, u64x4, f64x4 }
            $macro! { SfcAlt64x4j, u64x4, f64x4 }
            $macro! { SfcAlt64x4k, u64x4, f64x4 }
            $macro! { SfcAlt64x4l, u64x4, f64x4 }
            $macro! { SfcAlt64x8a, u64x8, f64x8 }
            $macro! { SfcAlt64x8b, u64x8, f64x8 }
            $macro! { SfcAlt64x8c, u64x8, f64x8 }
            $macro! { SfcAlt64x8d, u64x8, f64x8 }
            $macro! { SfcAlt64x8e, u64x8, f64x8 }
            $macro! { SfcAlt64x8f, u64x8, f64x8 }
            $macro! { SfcAlt64x8g, u64x8, f64x8 }
            $macro! { SfcAlt64x8h, u64x8, f64x8 }
            $macro! { SfcAlt64x8i, u64x8, f64x8 }
            $macro! { SfcAlt64x8j, u64x8, f64x8 }
            $macro! { SfcAlt64x8k, u64x8, f64x8 }
            $macro! { SfcAlt64x8l, u64x8, f64x8 }
            $macro! { SfcAlt32x2a, u32x2, f32x2 }
            $macro! { SfcAlt32x2b, u32x2, f32x2 }
            $macro! { SfcAlt32x2c, u32x2, f32x2 }
            $macro! { SfcAlt32x2d, u32x2, f32x2 }
            $macro! { SfcAlt32x2e, u32x2, f32x2 }
            $macro! { SfcAlt32x2f, u32x2, f32x2 }
            $macro! { SfcAlt32x2g, u32x2, f32x2 }
            $macro! { SfcAlt32x2h, u32x2, f32x2 }
            $macro! { SfcAlt32x2i, u32x2, f32x2 }
            $macro! { SfcAlt32x2j, u32x2, f32x2 }
            $macro! { SfcAlt32x2k, u32x2, f32x2 }
            $macro! { SfcAlt32x2l, u32x2, f32x2 }
            $macro! { SfcAlt32x4a, u32x4, f32x4 }
            $macro! { SfcAlt32x4b, u32x4, f32x4 }
            $macro! { SfcAlt32x4c, u32x4, f32x4 }
            $macro! { SfcAlt32x4d, u32x4, f32x4 }
            $macro! { SfcAlt32x4e, u32x4, f32x4 }
            $macro! { SfcAlt32x4f, u32x4, f32x4 }
            $macro! { SfcAlt32x4g, u32x4, f32x4 }
            $macro! { SfcAlt32x4h, u32x4, f32x4 }
            $macro! { SfcAlt32x4i, u32x4, f32x4 }
            $macro! { SfcAlt32x4j, u32x4, f32x4 }
            $macro! { SfcAlt32x4k, u32x4, f32x4 }
            $macro! { SfcAlt32x4l, u32x4, f32x4 }
            $macro! { SfcAlt32x8a, u32x8, f32x8 }
            $macro! { SfcAlt32x8b, u32x8, f32x8 }
            $macro! { SfcAlt32x8c, u32x8, f32x8 }
            $macro! { SfcAlt32x8d, u32x8, f32x8 }
            $macro! { SfcAlt32x8e, u32x8, f32x8 }
            $macro! { SfcAlt32x8f, u32x8, f32x8 }
            $macro! { SfcAlt32x8g, u32x8, f32x8 }
            $macro! { SfcAlt32x8h, u32x8, f32x8 }
            $macro! { SfcAlt32x8i, u32x8, f32x8 }
            $macro! { SfcAlt32x8j, u32x8, f32x8 }
            $macro! { SfcAlt32x8k, u32x8, f32x8 }
            $macro! { SfcAlt32x8l, u32x8, f32x8 }
            $macro! { SfcAlt32x16a, u32x16, f32x16 }
            $macro! { SfcAlt32x16b, u32x16, f32x16 }
            $macro! { SfcAlt32x16c, u32x16, f32x16 }
            $macro! { SfcAlt32x16d, u32x16, f32x16 }
            $macro! { SfcAlt32x16e, u32x16, f32x16 }
            $macro! { SfcAlt32x16f, u32x16, f32x16 }
            $macro! { SfcAlt32x16g, u32x16, f32x16 }
            $macro! { SfcAlt32x16h, u32x16, f32x16 }
            $macro! { SfcAlt32x16i, u32x16, f32x16 }
            $macro! { SfcAlt32x16j, u32x16, f32x16 }
            $macro! { SfcAlt32x16k, u32x16, f32x16 }
            $macro! { SfcAlt32x16l, u32x16, f32x16 }
            $macro! { SfcAlt16x2a, u16x2, f32x2 }
            $macro! { SfcAlt16x2b, u16x2, f32x2 }
            $macro! { SfcAlt16x2c, u16x2, f32x2 }
            $macro! { SfcAlt16x2d, u16x2, f32x2 }
            $macro! { SfcAlt16x2e, u16x2, f32x2 }
            $macro! { SfcAlt16x2f, u16x2, f32x2 }
            $macro! { SfcAlt16x2g, u16x2, f32x2 }
            $macro! { SfcAlt16x2h, u16x2, f32x2 }
            $macro! { SfcAlt16x2i, u16x2, f32x2 }
            $macro! { SfcAlt16x2j, u16x2, f32x2 }
            $macro! { SfcAlt16x2k, u16x2, f32x2 }
            $macro! { SfcAlt16x2l, u16x2, f32x2 }
            $macro! { SfcAlt16x4a, u16x4, f32x2 }
            $macro! { SfcAlt16x4b, u16x4, f32x2 }
            $macro! { SfcAlt16x4c, u16x4, f32x2 }
            $macro! { SfcAlt16x4d, u16x4, f32x2 }
            $macro! { SfcAlt16x4e, u16x4, f32x2 }
            $macro! { SfcAlt16x4f, u16x4, f32x2 }
            $macro! { SfcAlt16x4g, u16x4, f32x2 }
            $macro! { SfcAlt16x4h, u16x4, f32x2 }
            $macro! { SfcAlt16x4i, u16x4, f32x2 }
            $macro! { SfcAlt16x4j, u16x4, f32x2 }
            $macro! { SfcAlt16x4k, u16x4, f32x2 }
            $macro! { SfcAlt16x4l, u16x4, f32x2 }
            $macro! { SfcAlt16x8a, u16x8, f32x4 }
            $macro! { SfcAlt16x8b, u16x8, f32x4 }
            $macro! { SfcAlt16x8c, u16x8, f32x4 }
            $macro! { SfcAlt16x8d, u16x8, f32x4 }
            $macro! { SfcAlt16x8e, u16x8, f32x4 }
            $macro! { SfcAlt16x8f, u16x8, f32x4 }
            $macro! { SfcAlt16x8g, u16x8, f32x4 }
            $macro! { SfcAlt16x8h, u16x8, f32x4 }
            $macro! { SfcAlt16x8i, u16x8, f32x4 }
            $macro! { SfcAlt16x8j, u16x8, f32x4 }
            $macro! { SfcAlt16x8k, u16x8, f32x4 }
            $macro! { SfcAlt16x8l, u16x8, f32x4 }
            $macro! { SfcAlt16x16a, u16x16, f32x8 }
            $macro! { SfcAlt16x16b, u16x16, f32x8 }
            $macro! { SfcAlt16x16c, u16x16, f32x8 }
            $macro! { SfcAlt16x16d, u16x16, f32x8 }
            $macro! { SfcAlt16x16e, u16x16, f32x8 }
            $macro! { SfcAlt16x16f, u16x16, f32x8 }
            $macro! { SfcAlt16x16g, u16x16, f32x8 }
            $macro! { SfcAlt16x16h, u16x16, f32x8 }
            $macro! { SfcAlt16x16i, u16x16, f32x8 }
            $macro! { SfcAlt16x16j, u16x16, f32x8 }
            $macro! { SfcAlt16x16k, u16x16, f32x8 }
            $macro! { SfcAlt16x16l, u16x16, f32x8 }
            $macro! { SfcAlt16x32a, u16x32, f32x16 }
            $macro! { SfcAlt16x32b, u16x32, f32x16 }
            $macro! { SfcAlt16x32c, u16x32, f32x16 }
            $macro! { SfcAlt16x32d, u16x32, f32x16 }
            $macro! { SfcAlt16x32e, u16x32, f32x16 }
            $macro! { SfcAlt16x32f, u16x32, f32x16 }
            $macro! { SfcAlt16x32g, u16x32, f32x16 }
            $macro! { SfcAlt16x32h, u16x32, f32x16 }
            $macro! { SfcAlt16x32i, u16x32, f32x16 }
            $macro! { SfcAlt16x32j, u16x32, f32x16 }
            $macro! { SfcAlt16x32k, u16x32, f32x16 }
            $macro! { SfcAlt16x32l, u16x32, f32x16 }
            $macro! { SfcAlt8x2a, u8x2, f32x2 }
            $macro! { SfcAlt8x2b, u8x2, f32x2 }
            $macro! { SfcAlt8x2c, u8x2, f32x2 }
            $macro! { SfcAlt8x2d, u8x2, f32x2 }
            $macro! { SfcAlt8x2e, u8x2, f32x2 }
            $macro! { SfcAlt8x2f, u8x2, f32x2 }
            $macro! { SfcAlt8x2g, u8x2, f32x2 }
            $macro! { SfcAlt8x2h, u8x2, f32x2 }
            $macro! { SfcAlt8x2i, u8x2, f32x2 }
            $macro! { SfcAlt8x2j, u8x2, f32x2 }
            $macro! { SfcAlt8x2k, u8x2, f32x2 }
            $macro! { SfcAlt8x2l, u8x2, f32x2 }
            $macro! { SfcAlt8x4a, u8x4, f32x2 }
            $macro! { SfcAlt8x4b, u8x4, f32x2 }
            $macro! { SfcAlt8x4c, u8x4, f32x2 }
            $macro! { SfcAlt8x4d, u8x4, f32x2 }
            $macro! { SfcAlt8x4e, u8x4, f32x2 }
            $macro! { SfcAlt8x4f, u8x4, f32x2 }
            $macro! { SfcAlt8x4g, u8x4, f32x2 }
            $macro! { SfcAlt8x4h, u8x4, f32x2 }
            $macro! { SfcAlt8x4i, u8x4, f32x2 }
            $macro! { SfcAlt8x4j, u8x4, f32x2 }
            $macro! { SfcAlt8x4k, u8x4, f32x2 }
            $macro! { SfcAlt8x4l, u8x4, f32x2 }
            $macro! { SfcAlt8x8a, u8x8, f32x2 }
            $macro! { SfcAlt8x8b, u8x8, f32x2 }
            $macro! { SfcAlt8x8c, u8x8, f32x2 }
            $macro! { SfcAlt8x8d, u8x8, f32x2 }
            $macro! { SfcAlt8x8e, u8x8, f32x2 }
            $macro! { SfcAlt8x8f, u8x8, f32x2 }
            $macro! { SfcAlt8x8g, u8x8, f32x2 }
            $macro! { SfcAlt8x8h, u8x8, f32x2 }
            $macro! { SfcAlt8x8i, u8x8, f32x2 }
            $macro! { SfcAlt8x8j, u8x8, f32x2 }
            $macro! { SfcAlt8x8k, u8x8, f32x2 }
            $macro! { SfcAlt8x8l, u8x8, f32x2 }
            $macro! { SfcAlt8x16a, u8x16, f32x4 }
            $macro! { SfcAlt8x16b, u8x16, f32x4 }
            $macro! { SfcAlt8x16c, u8x16, f32x4 }
            $macro! { SfcAlt8x16d, u8x16, f32x4 }
            $macro! { SfcAlt8x16e, u8x16, f32x4 }
            $macro! { SfcAlt8x16f, u8x16, f32x4 }
            $macro! { SfcAlt8x16g, u8x16, f32x4 }
            $macro! { SfcAlt8x16h, u8x16, f32x4 }
            $macro! { SfcAlt8x16i, u8x16, f32x4 }
            $macro! { SfcAlt8x16j, u8x16, f32x4 }
            $macro! { SfcAlt8x16k, u8x16, f32x4 }
            $macro! { SfcAlt8x16l, u8x16, f32x4 }
            $macro! { SfcAlt8x32a, u8x32, f32x8 }
            $macro! { SfcAlt8x32b, u8x32, f32x8 }
            $macro! { SfcAlt8x32c, u8x32, f32x8 }
            $macro! { SfcAlt8x32d, u8x32, f32x8 }
            $macro! { SfcAlt8x32e, u8x32, f32x8 }
            $macro! { SfcAlt8x32f, u8x32, f32x8 }
            $macro! { SfcAlt8x32g, u8x32, f32x8 }
            $macro! { SfcAlt8x32h, u8x32, f32x8 }
            $macro! { SfcAlt8x32i, u8x32, f32x8 }
            $macro! { SfcAlt8x32j, u8x32, f32x8 }
            $macro! { SfcAlt8x32k, u8x32, f32x8 }
            $macro! { SfcAlt8x32l, u8x32, f32x8 }
            $macro! { SfcAlt8x64a, u8x64, f32x16 }
            $macro! { SfcAlt8x64b, u8x64, f32x16 }
            $macro! { SfcAlt8x64c, u8x64, f32x16 }
            $macro! { SfcAlt8x64d, u8x64, f32x16 }
            $macro! { SfcAlt8x64e, u8x64, f32x16 }
            $macro! { SfcAlt8x64f, u8x64, f32x16 }
            $macro! { SfcAlt8x64g, u8x64, f32x16 }
            $macro! { SfcAlt8x64h, u8x64, f32x16 }
            $macro! { SfcAlt8x64i, u8x64, f32x16 }
            $macro! { SfcAlt8x64j, u8x64, f32x16 }
            $macro! { SfcAlt8x64k, u8x64, f32x16 }
            $macro! { SfcAlt8x64l, u8x64, f32x16 }

            $macro! { VeryFast64x2a, u64x2, f64x2 }
            $macro! { VeryFast64x2b, u64x2, f64x2 }
            $macro! { VeryFast64x2c, u64x2, f64x2 }
            $macro! { VeryFast64x2d, u64x2, f64x2 }
            $macro! { VeryFast64x2e, u64x2, f64x2 }
            $macro! { VeryFast64x2f, u64x2, f64x2 }
            $macro! { VeryFast64x2g, u64x2, f64x2 }
            $macro! { VeryFast64x4a, u64x4, f64x4 }
            $macro! { VeryFast64x4b, u64x4, f64x4 }
            $macro! { VeryFast64x4c, u64x4, f64x4 }
            $macro! { VeryFast64x4d, u64x4, f64x4 }
            $macro! { VeryFast64x4e, u64x4, f64x4 }
            $macro! { VeryFast64x4f, u64x4, f64x4 }
            $macro! { VeryFast64x4g, u64x4, f64x4 }
            $macro! { VeryFast64x8a, u64x8, f64x8 }
            $macro! { VeryFast64x8b, u64x8, f64x8 }
            $macro! { VeryFast64x8c, u64x8, f64x8 }
            $macro! { VeryFast64x8d, u64x8, f64x8 }
            $macro! { VeryFast64x8e, u64x8, f64x8 }
            $macro! { VeryFast64x8f, u64x8, f64x8 }
            $macro! { VeryFast64x8g, u64x8, f64x8 }
            $macro! { VeryFast32x2a, u32x2, f32x2 }
            $macro! { VeryFast32x2b, u32x2, f32x2 }
            $macro! { VeryFast32x2c, u32x2, f32x2 }
            $macro! { VeryFast32x2d, u32x2, f32x2 }
            $macro! { VeryFast32x2e, u32x2, f32x2 }
            $macro! { VeryFast32x2f, u32x2, f32x2 }
            $macro! { VeryFast32x2g, u32x2, f32x2 }
            $macro! { VeryFast32x4a, u32x4, f32x4 }
            $macro! { VeryFast32x4b, u32x4, f32x4 }
            $macro! { VeryFast32x4c, u32x4, f32x4 }
            $macro! { VeryFast32x4d, u32x4, f32x4 }
            $macro! { VeryFast32x4e, u32x4, f32x4 }
            $macro! { VeryFast32x4f, u32x4, f32x4 }
            $macro! { VeryFast32x4g, u32x4, f32x4 }
            $macro! { VeryFast32x8a, u32x8, f32x8 }
            $macro! { VeryFast32x8b, u32x8, f32x8 }
            $macro! { VeryFast32x8c, u32x8, f32x8 }
            $macro! { VeryFast32x8d, u32x8, f32x8 }
            $macro! { VeryFast32x8e, u32x8, f32x8 }
            $macro! { VeryFast32x8f, u32x8, f32x8 }
            $macro! { VeryFast32x8g, u32x8, f32x8 }
            $macro! { VeryFast32x16a, u32x16, f32x16 }
            $macro! { VeryFast32x16b, u32x16, f32x16 }
            $macro! { VeryFast32x16c, u32x16, f32x16 }
            $macro! { VeryFast32x16d, u32x16, f32x16 }
            $macro! { VeryFast32x16e, u32x16, f32x16 }
            $macro! { VeryFast32x16f, u32x16, f32x16 }
            $macro! { VeryFast32x16g, u32x16, f32x16 }
            $macro! { VeryFast16x2a, u16x2, f32x2 }
            $macro! { VeryFast16x2b, u16x2, f32x2 }
            $macro! { VeryFast16x2c, u16x2, f32x2 }
            $macro! { VeryFast16x2d, u16x2, f32x2 }
            $macro! { VeryFast16x2e, u16x2, f32x2 }
            $macro! { VeryFast16x2f, u16x2, f32x2 }
            $macro! { VeryFast16x2g, u16x2, f32x2 }
            $macro! { VeryFast16x4a, u16x4, f32x2 }
            $macro! { VeryFast16x4b, u16x4, f32x2 }
            $macro! { VeryFast16x4c, u16x4, f32x2 }
            $macro! { VeryFast16x4d, u16x4, f32x2 }
            $macro! { VeryFast16x4e, u16x4, f32x2 }
            $macro! { VeryFast16x4f, u16x4, f32x2 }
            $macro! { VeryFast16x4g, u16x4, f32x2 }
            $macro! { VeryFast16x8a, u16x8, f32x4 }
            $macro! { VeryFast16x8b, u16x8, f32x4 }
            $macro! { VeryFast16x8c, u16x8, f32x4 }
            $macro! { VeryFast16x8d, u16x8, f32x4 }
            $macro! { VeryFast16x8e, u16x8, f32x4 }
            $macro! { VeryFast16x8f, u16x8, f32x4 }
            $macro! { VeryFast16x8g, u16x8, f32x4 }
            $macro! { VeryFast16x16a, u16x16, f32x8 }
            $macro! { VeryFast16x16b, u16x16, f32x8 }
            $macro! { VeryFast16x16c, u16x16, f32x8 }
            $macro! { VeryFast16x16d, u16x16, f32x8 }
            $macro! { VeryFast16x16e, u16x16, f32x8 }
            $macro! { VeryFast16x16f, u16x16, f32x8 }
            $macro! { VeryFast16x16g, u16x16, f32x8 }
            $macro! { VeryFast16x32a, u16x32, f32x16 }
            $macro! { VeryFast16x32b, u16x32, f32x16 }
            $macro! { VeryFast16x32c, u16x32, f32x16 }
            $macro! { VeryFast16x32d, u16x32, f32x16 }
            $macro! { VeryFast16x32e, u16x32, f32x16 }
            $macro! { VeryFast16x32f, u16x32, f32x16 }
            $macro! { VeryFast16x32g, u16x32, f32x16 }
            $macro! { VeryFast8x2a, u8x2, f32x2 }
            $macro! { VeryFast8x2b, u8x2, f32x2 }
            $macro! { VeryFast8x2c, u8x2, f32x2 }
            $macro! { VeryFast8x2d, u8x2, f32x2 }
            $macro! { VeryFast8x2e, u8x2, f32x2 }
            $macro! { VeryFast8x2f, u8x2, f32x2 }
            $macro! { VeryFast8x2g, u8x2, f32x2 }
            $macro! { VeryFast8x4a, u8x4, f32x2 }
            $macro! { VeryFast8x4b, u8x4, f32x2 }
            $macro! { VeryFast8x4c, u8x4, f32x2 }
            $macro! { VeryFast8x4d, u8x4, f32x2 }
            $macro! { VeryFast8x4e, u8x4, f32x2 }
            $macro! { VeryFast8x4f, u8x4, f32x2 }
            $macro! { VeryFast8x4g, u8x4, f32x2 }
            $macro! { VeryFast8x8a, u8x8, f32x2 }
            $macro! { VeryFast8x8b, u8x8, f32x2 }
            $macro! { VeryFast8x8c, u8x8, f32x2 }
            $macro! { VeryFast8x8d, u8x8, f32x2 }
            $macro! { VeryFast8x8e, u8x8, f32x2 }
            $macro! { VeryFast8x8f, u8x8, f32x2 }
            $macro! { VeryFast8x8g, u8x8, f32x2 }
            $macro! { VeryFast8x16a, u8x16, f32x4 }
            $macro! { VeryFast8x16b, u8x16, f32x4 }
            $macro! { VeryFast8x16c, u8x16, f32x4 }
            $macro! { VeryFast8x16d, u8x16, f32x4 }
            $macro! { VeryFast8x16e, u8x16, f32x4 }
            $macro! { VeryFast8x16f, u8x16, f32x4 }
            $macro! { VeryFast8x16g, u8x16, f32x4 }
            $macro! { VeryFast8x32a, u8x32, f32x8 }
            $macro! { VeryFast8x32b, u8x32, f32x8 }
            $macro! { VeryFast8x32c, u8x32, f32x8 }
            $macro! { VeryFast8x32d, u8x32, f32x8 }
            $macro! { VeryFast8x32e, u8x32, f32x8 }
            $macro! { VeryFast8x32f, u8x32, f32x8 }
            $macro! { VeryFast8x32g, u8x32, f32x8 }
            $macro! { VeryFast8x64a, u8x64, f32x16 }
            $macro! { VeryFast8x64b, u8x64, f32x16 }
            $macro! { VeryFast8x64c, u8x64, f32x16 }
            $macro! { VeryFast8x64d, u8x64, f32x16 }
            $macro! { VeryFast8x64e, u8x64, f32x16 }
            $macro! { VeryFast8x64f, u8x64, f32x16 }
            $macro! { VeryFast8x64g, u8x64, f32x16 }
        }
    };
}
