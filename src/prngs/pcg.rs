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
                // We could easily use different parameters per stream here
                self.state = oldstate * 6364136223846793005 + self.inc;
                // Calculate output function (XSH RR), uses old state for max ILP
                let xorshifted: $vec32 = (((oldstate >> 18) ^ oldstate) >> 27).cast();
                let rot: $vec32 = (oldstate >> 59).cast();

                // This rotate could be replaced by a similarly functioning
                // vector shuffle on older hardware (24 possible shuffles,
                // 6 with "rotate one byte right" behavior)
                // something of the form of:
                /*let shuffles = (rotate_lane_by_0_bytes, rotate_lane_by_1_bytes, ...);
                let shuffle_indices = shuffle_by(shuffles, rot);
                let result = shuffle_by(xorshifted, shuffle_indices);*/

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

// (where `l` is stream length)
// (multiple parameters could be used)
// (stream selection is possible)
// Listing probability of overlap somewhere:            Probability
make_pcg! { Pcg32x2, u64x2, u32x2 } // 2^2 * l / 2^64 ≈ l * 2^-62
make_pcg! { Pcg32x4, u64x4, u32x4 } // 4^2 * l / 2^64 ≈ l * 2^-60
make_pcg! { Pcg32x8, u64x8, u32x8 } // 8^2 * l / 2^64 ≈ l * 2^-58
