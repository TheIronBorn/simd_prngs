//! 32-bit and 16-bit LCG. This is simply PCG with the bit-level ops removed.
//! Faster and presumably lower quality.

use rng_impl::*;

macro_rules! make_lcg {
    ($rng_name:ident, $vector:ident, $half:ident, $mul:expr) => {
        pub struct $rng_name {
            state: $vector,
            inc: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $half {
                let oldstate = self.state;
                // Advance internal state
                // We could easily use different parameters per stream here
                self.state = oldstate * $mul + self.inc;
                let r: $half = oldstate.cast();
                r
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

                lcg.state = lcg.state * $mul + lcg.inc;

                Ok(lcg)
            }
        }
    };

    ( 32_bit_out: $rng_name:ident, $vector:ident, $half:ident) => {
        make_lcg! { $rng_name, $vector, $half, 6364136223846793005 }
    };

    // 64-bit SIMD multiplication is less supported than 32-bit. These PRNGs
    // might be faster
    ( 16_bit_out: $rng_name:ident, $vector:ident, $half:ident) => {
        make_lcg! { $rng_name, $vector, $half, 747796405 }
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
// (where `l` is stream length)
// (multiple parameters could be used)
// Listing probability of overlap somewhere:                            Probability
make_lcg! { 32_bit_out: Lcg32x2, u64x2, u32x2 } // 2^2 * l / 2^64 ≈     l * 2^-62
make_lcg! { 32_bit_out: Lcg32x4, u64x4, u32x4 } // 4^2 * l / 2^64 ≈     l * 2^-60
make_lcg! { 32_bit_out: Lcg32x8, u64x8, u32x8 } // 8^2 * l / 2^64 ≈     l * 2^-58

make_lcg! { 16_bit_out: Lcg16x2, u32x2, u16x2 } // 2^2 * l / 2^32 ≈     l * 2^-30
make_lcg! { 16_bit_out: Lcg16x4, u32x4, u16x4 } // 4^2 * l / 2^32 ≈     l * 2^-28
make_lcg! { 16_bit_out: Lcg16x8, u32x8, u16x8 } // 8^2 * l / 2^32 ≈     l * 2^-26
make_lcg! { 16_bit_out: Lcg16x16, u32x16, u16x16 } // 16^2 * l / 2^32 ≈ l * 2^-24
