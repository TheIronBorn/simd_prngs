use std::simd::*;

use rng_impl::*;

macro_rules! make_xoroshiro {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                let s0 = self.s0;
                let mut s1 = self.s1;
                // The `++` scrambler might be faster (multiplication,
                // particularly 64-bit, is slow with SIMD). The paper suggests
                // this rotate could be replaced by `x ^= x >> 7`.
                let result = (s0 * 5).rotate_left(7) * 9;

                s1 ^= s0;
                // this rotate could be implemented as a shuffle (as it is
                // divisible by 8)
                self.s0 = s0.rotate_left(24) ^ s1 ^ (s1 << 16); // a, b
                self.s1 = s1.rotate_left(37); // c

                result
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                const ZERO: $vector = $vector::splat(0);

                let mut seeds = [$vector::default(); 2];
                while seeds
                    .iter()
                    // `splat(true)`
                    .fold(ZERO.eq(ZERO), |acc, s| acc & s.eq(&ZERO))
                    .any()
                {
                    rng.try_fill(seeds.as_byte_slice_mut())?;
                }

                Ok(Self {
                    s0: seeds[0],
                    s1: seeds[1],
                })
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (jumping is possible)
// Listing probability of overlap somewhere:               Probability
make_xoroshiro! { Xoroshiro128StarStarX2, u64x2 } // 2^2 * l / 2^128 = l * 2^-126
make_xoroshiro! { Xoroshiro128StarStarX4, u64x4 } // 4^2 * l / 2^128 = l * 2^-124
make_xoroshiro! { Xoroshiro128StarStarX8, u64x8 } // 8^2 * l / 2^128 = l * 2^-122
