use rng_impl::*;

macro_rules! make_jsf_32 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            a: $vector,
            b: $vector,
            c: $vector,
            d: $vector,
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                // Alternate rotations allow vector shuffle rotation optimization. Similar
                // parameters probably exist for the 64-bit variant.
                // Canonical: 27, 17
                // Other sets that achieve 8.8 bits of avalanche include (9,16), (9,24),
                // (10,16), (10,24), (11,16), (11,24), (25,8), (25,16), (26,8), (26,16),
                // (26,17), and (27,16).
                let e = self.a - self.b.rotate_left_opt(9);
                self.a = self.b ^ self.c.rotate_left_opt(16);
                self.b = self.c + self.d;
                self.c = self.d + e;
                self.d = e + self.a;
                self.d
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 4];
                rng.try_fill_bytes(seed.as_byte_slice_mut())?;

                let a = seed[0];
                let b = seed[1];
                let c = seed[2];
                let mut d = seed[3];

                // PractRand: block the cycles of length 1
                let flag = (d & 0x80093300).eq($vector::splat(0));

                macro_rules! all_eq {
                    ($const_0:expr, $const_1:expr, $const_2:expr, $const_3:expr) => {
                        a.eq($vector::splat($const_0))
                            & b.eq($vector::splat($const_1))
                            & c.eq($vector::splat($const_2))
                            & d.eq($vector::splat($const_3))
                    };
                }

                macro_rules! select_incr {
                    ($m:expr) => {{
                        ($m & flag).select($vector::splat(1), $vector::splat(0))
                    }};
                }

                d += select_incr!($vector::splat(0).ne(!(a | b | c | d)));
                d += select_incr!(all_eq!(0x77777777, 0x55555555, 0x11111111, 0x44444444));
                d += select_incr!(all_eq!(0x5591F2E3, 0x69EBA6CD, 0x2A171E3D, 0x3FD48890));
                d += select_incr!(all_eq!(0x47CB8D56, 0xAE9B35A7, 0x5C78F4A8, 0x522240FF));

                Ok(Self { a, b, c, d })
            }
        }
    };
}

// (where `l` is stream length)
// (using average cycle length)
// (multiple parameters could be used, though slow on older hardware)
// Listing probability of overlap somewhere:              Probability
make_jsf_32! { Jsf32x2,  u32x2  } // ≈ 2^2  * l / 2^127 ≈ l * 2^-125
make_jsf_32! { Jsf32x4,  u32x4  } // ≈ 4^2  * l / 2^127 ≈ l * 2^-123
make_jsf_32! { Jsf32x8,  u32x8  } // ≈ 8^2  * l / 2^127 ≈ l * 2^-121
make_jsf_32! { Jsf32x16, u32x16 } // ≈ 16^2 * l / 2^127 ≈ l * 2^-119

macro_rules! make_jsf_64 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            a: $vector,
            b: $vector,
            c: $vector,
            d: $vector,
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                let e = self.a - self.b.rotate_left_opt(39);
                self.a = self.b ^ self.c.rotate_left_opt(11);
                self.b = self.c + self.d;
                self.c = self.d + e;
                self.d = e + self.a;
                self.d
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 4];
                rng.try_fill_bytes(seed.as_byte_slice_mut())?;

                let a = seed[0];
                let b = seed[1];
                let c = seed[2];
                let mut d = seed[3];

                // necessary when we can assume a good seed from the seeding RNG?
                let flag = (!(a | b | c | d)).ne($vector::splat(0));
                d += flag.select($vector::splat(1), $vector::splat(0));

                Ok(Self { a, b, c, d })
            }
        }
    };
}

make_jsf_64! { Jsf64x2, u64x2 } // ≈ 2^2 * l / 2^255 ≈  l * 2^-253
make_jsf_64! { Jsf64x4, u64x4 } // ≈ 4^2 * l / 2^255 ≈  l * 2^-251
make_jsf_64! { Jsf64x8, u64x8 } // ≈ 8^2 * l / 2^255 ≈  l * 2^-249
