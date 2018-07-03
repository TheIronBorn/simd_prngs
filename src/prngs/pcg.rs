use std::simd::*;

use rng_impl::*;

macro_rules! make_pcg {
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
                // Calculate output function (XSH RR), uses old state for max ILP
                let xorshifted = $vec32::from(((oldstate >> 18) ^ oldstate) >> 27);
                let rot = $vec32::from(oldstate >> 59);

                // This rotate could be replaced by a similarly functioning
                // vector shuffle on older hardware (24 possible shuffles,
                // 6 with "rotate one bit right" behavior)
                // xorshifted.rotate_right(rot)
                (xorshifted >> rot) | (xorshifted << (32 - rot))
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

                let mut pcg = Self {
                    state: seed[0],
                    inc: seed[1] | 1, // must be odd
                };

                pcg.state = pcg.state * 6364136223846793005 + pcg.inc;

                Ok(pcg)
            }
        }
    };
}

make_pcg! { Pcg32x2, u64x2, u32x2 }
make_pcg! { Pcg32x4, u64x4, u32x4 }
make_pcg! { Pcg32x8, u64x8, u32x8 }
