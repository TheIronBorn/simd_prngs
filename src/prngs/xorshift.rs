use rng_impl::*;

macro_rules! make_xorshift {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            x: $vector,
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                self.x ^= self.x << 13;
                self.x ^= self.x >> 17;
                self.x ^= self.x << 5;
                self.x
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 1];
                rng.try_fill_bytes(seed.as_byte_slice_mut())?;

                while seed[0].eq($vector::splat(0)).any() {
                    rng.try_fill_bytes(seed.as_byte_slice_mut())?;
                }

                Ok(Self { x: seed[0] })
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (jumping is possible)
#[rustfmt::skip]
// Listing probability of overlap somewhere:                                 Probability
make_xorshift! { Xorshift32x2,  u32x2  } // ≈ 2^2  * l / 2^32 ≈ l * 2^-30
make_xorshift! { Xorshift32x4,  u32x4  } // ≈ 4^2  * l / 2^32 ≈ l * 2^-28
make_xorshift! { Xorshift32x8,  u32x8  } // ≈ 8^2  * l / 2^32 ≈ l * 2^-26
make_xorshift! { Xorshift32x16, u32x16 } // ≈ 16^2 * l / 2^32 ≈ l * 2^-24

macro_rules! make_xorshift128 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            x: $vector,
            y: $vector,
            z: $vector,
            w: $vector,
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                let x = self.x;
                let t = x ^ (x << 11);
                self.x = self.y;
                self.y = self.z;
                self.z = self.w;
                let w_ = self.w;
                self.w = w_ ^ (w_ >> 19) ^ (t ^ (t >> 8));
                self.w
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seeds = [$vector::default(); 4];
                // while any lane has all zero seeds
                while seeds
                    .iter()
                    .fold($vector::splat(0), |mask, &s| mask | s)
                    .eq($vector::splat(0))
                    .any()
                {
                    rng.try_fill_bytes(seeds.as_byte_slice_mut())?;
                }

                Ok(Self {
                    x: seeds[0],
                    y: seeds[1],
                    z: seeds[2],
                    w: seeds[3],
                })
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (jumping is possible)
#[rustfmt::skip]
// Listing probability of overlap somewhere:                         Probability
make_xorshift128! { Xorshift128x2,  u32x2  } // ≈ 2^2  * l / 2^128 ≈ l * 2^-126
make_xorshift128! { Xorshift128x4,  u32x4  } // ≈ 4^2  * l / 2^128 ≈ l * 2^-124
make_xorshift128! { Xorshift128x8,  u32x8  } // ≈ 8^2  * l / 2^128 ≈ l * 2^-122
make_xorshift128! { Xorshift128x16, u32x16 } // ≈ 16^2 * l / 2^128 ≈ l * 2^-120
