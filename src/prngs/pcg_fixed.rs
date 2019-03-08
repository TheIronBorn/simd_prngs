//! Pcg32 using the "XS[H/l] -- fixed xorshift" permutations. For speed
//! comparison only. It likely has poor statistical quality.
//!
//! PcgFixedXsh32x2 reached 1TB with PractRand

use rng_impl::*;

macro_rules! make_pcg_xsh {
    ($rng_name:ident, $vector:ident, $vec32:ident) => {
        pub struct $rng_name {
            state: $vector,
            inc: $vector,
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vec32;

            #[inline(always)]
            fn generate(&mut self) -> $vec32 {
                let mut oldstate = self.state;
                // Advance internal state
                self.state = oldstate * 6364136223846793005 + self.inc;

                const XTYPE_BITS: u32 = 32;
                const BITS: u32 = 64;
                const SPARE_BITS: u32 = BITS - XTYPE_BITS;
                const TOP_SPARE: u32 = 0;
                const BOTTOM_SPARE: u32 = SPARE_BITS - TOP_SPARE;
                const XSHIFT: u32 = (TOP_SPARE + XTYPE_BITS) / 2;

                oldstate ^= oldstate >> XSHIFT;
                (oldstate >> BOTTOM_SPARE).cast()
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 2];
                rng.try_fill_bytes(seed.as_byte_slice_mut())?;

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
#[rustfmt::skip]
// Listing probability of overlap somewhere:                          Probability
make_pcg_xsh! { PcgFixedXsh32x2, u64x2, u32x2 } // ≈ 2^2 * l / 2^32 ≈ l * 2^-30
make_pcg_xsh! { PcgFixedXsh32x4, u64x4, u32x4 } // ≈ 4^2 * l / 2^32 ≈ l * 2^-28
make_pcg_xsh! { PcgFixedXsh32x8, u64x8, u32x8 } // ≈ 8^2 * l / 2^32 ≈ l * 2^-26

macro_rules! make_pcg_xsl {
    ($rng_name:ident, $vector:ident, $vec32:ident) => {
        pub struct $rng_name {
            state: $vector,
            inc: $vector,
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vec32;

            #[inline(always)]
            fn generate(&mut self) -> $vec32 {
                let mut oldstate = self.state;
                // Advance internal state
                self.state = oldstate * 6364136223846793005 + self.inc;

                const XTYPE_BITS: u32 = 32;
                const BITS: u32 = 64;
                const SPARE_BITS: u32 = BITS - XTYPE_BITS;
                const TOP_SPARE: u32 = SPARE_BITS;
                const BOTTOM_SPARE: u32 = SPARE_BITS - TOP_SPARE;
                const XSHIFT: u32 = (TOP_SPARE + XTYPE_BITS) / 2;

                oldstate ^= oldstate >> XSHIFT;
                (oldstate >> BOTTOM_SPARE).cast()
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 2];
                rng.try_fill_bytes(seed.as_byte_slice_mut())?;

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
#[rustfmt::skip]
// Listing probability of overlap somewhere:                          Probability
make_pcg_xsl! { PcgFixedXsl32x2, u64x2, u32x2 } // ≈ 2^2 * l / 2^64 ≈ l * 2^-62
make_pcg_xsl! { PcgFixedXsl32x4, u64x4, u32x4 } // ≈ 4^2 * l / 2^64 ≈ l * 2^-60
make_pcg_xsl! { PcgFixedXsl32x8, u64x8, u32x8 } // ≈ 8^2 * l / 2^64 ≈ l * 2^-58
