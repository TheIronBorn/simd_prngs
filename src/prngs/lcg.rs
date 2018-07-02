//! 32-bit LCG. This is simply PCG with the bit-level ops removed.
//! Faster and presumably lower quality.

use std::simd::*;

use rng_impl::*;

macro_rules! make_lcg {
    ($rng_name:ident, $vector:ident, $vec32:ident) => {
        pub struct $rng_name {
            state: $vector,
            inc: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vec32 {
                let oldstate = self.state;
                // Advance internal state
                self.state = oldstate * 6364136223846793005 + self.inc;
                $vec32::from(oldstate)
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 2];
                rng.try_fill(seed.as_byte_slice_mut())?;

                let mut lcg = Self {
                    state: seed[0],
                    inc: seed[1] | 1, // must be odd
                };

                lcg.state = lcg.state * 6364136223846793005 + lcg.inc;

                Ok(lcg)
            }
        }
    };
}

make_lcg! { Lcg32x2, u64x2, u32x2 }
make_lcg! { Lcg32x4, u64x4, u32x4 }
make_lcg! { Lcg32x8, u64x8, u32x8 }
