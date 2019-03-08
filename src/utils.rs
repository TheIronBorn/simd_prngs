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
    };
}
