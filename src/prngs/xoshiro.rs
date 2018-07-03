use std::simd::*;

use rng_impl::*;

macro_rules! make_xoshiro {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
            s2: $vector,
            s3: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // The `++` scrambler might be faster. The paper suggests this
                // rotate could be replaced by `x ^= x >> 7`.
                let result_starstar = (self.s1 * 5).rotate_left(7) * 9;

                let t = self.s1 << 17;

                self.s2 ^= self.s0;
                self.s3 ^= self.s1;
                self.s1 ^= self.s2;
                self.s0 ^= self.s3;

                self.s2 ^= t;

                self.s3 = self.s3.rotate_left(45);

                result_starstar
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

                let mut seeds = [$vector::default(); 4];
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
                    s2: seeds[2],
                    s3: seeds[3],
                })
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (jumping is possible)
// Listing probability of overlap somewhere:                       Probability
make_xoshiro! { Xoshiro256StarStarX2, u64x2 } // 2^2 * l / 2^256 = l * 2^-254
make_xoshiro! { Xoshiro256StarStarX4, u64x4 } // 4^2 * l / 2^256 = l * 2^-252
make_xoshiro! { Xoshiro256StarStarX8, u64x8 } // 8^2 * l / 2^256 = l * 2^-250
