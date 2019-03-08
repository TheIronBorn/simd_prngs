use rng_impl::*;

macro_rules! make_xorshift128plus {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                let mut s1 = self.s0;
                let s0 = self.s1;
                // replacing this scrambler with one from the "Scrambled Linear
                // Pseudorandom Number Generators" paper might provide a
                // sufficient balance of speed/quality
                let result = s0 + s1;
                self.s0 = s0;
                s1 ^= s1 << 23; // a
                self.s1 = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5); // b, c
                result
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seeds = [$vector::default(); 2];
                while seeds
                    .iter()
                    .fold($vector::splat(0), |mask, &s| mask | s)
                    .eq($vector::splat(0))
                    .any()
                {
                    rng.try_fill_bytes(seeds.as_byte_slice_mut())?;
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
#[rustfmt::skip]
// Listing probability of overlap somewhere:                              Probability
make_xorshift128plus! { Xorshift128PlusX2, u64x2 } // ≈ 2^2 * l / 2^128 ≈ l * 2^-126
make_xorshift128plus! { Xorshift128PlusX4, u64x4 } // ≈ 4^2 * l / 2^128 ≈ l * 2^-124
make_xorshift128plus! { Xorshift128PlusX8, u64x8 } // ≈ 8^2 * l / 2^128 ≈ l * 2^-122
