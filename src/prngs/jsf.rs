use std::simd::*;

use rng_impl::*;

// vertical vector rotations can be better implemented with vector
// shuffles when the rotate distance is a multiple of 8
macro_rules! rotate_left {
    ($x:expr, 16, u32x2) => {{
        const ROTL_16: [u32; 16] = [2, 3, 0, 1, 6, 7, 4, 5];
        let vec8 = u8x8::from_bits($x);
        let r: u8x8 = unsafe { simd_shuffle8(vec8, vec8, ROTL_16) };
        u32x2::from_bits(r)
    }};
    ($x:expr, 16, u32x4) => {{
        const ROTL_16: [u32; 16] = [2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13];
        let vec8 = u8x16::from_bits($x);
        let r: u8x16 = unsafe { simd_shuffle16(vec8, vec8, ROTL_16) };
        u32x4::from_bits(r)
    }};
    ($x:expr, 16, u32x8) => {{
        const ROTL_16: [u32; 16] = [2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13, 18, 19, 16, 17, 22, 23, 20, 21, 26, 27, 24, 25, 30, 31, 28, 29];
        let vec8 = u8x32::from_bits($x);
        let r: u8x32 = unsafe { simd_shuffle32(vec8, vec8, ROTL_16) };
        u32x8::from_bits(r)
    }};
    ($x:expr, 16, u32x16) => {{
        const ROTL_16: [u32; 16] = [2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13, 18, 19, 16, 17, 22, 23, 20, 21, 26, 27, 24, 25, 30, 31, 28, 29, 34, 35, 32, 33, 38, 39, 36, 37, 42, 43, 40, 41, 46, 47, 44, 45, 50, 51, 48, 49, 54, 55, 52, 53, 58, 59, 56, 57, 62, 63, 60, 61];
        let vec8 = u8x64::from_bits($x);
        let r: u8x64 = unsafe { simd_shuffle64(vec8, vec8, ROTL_16) };
        u32x16::from_bits(r)
    }};
    ($x:expr, $rot:expr, $y:ident) => {{
        $x.rotate_left($rot)
    }};
}

macro_rules! make_jsf {
    ($rng_name:ident, $vector:ident, $x:expr, $z:expr) => {
        pub struct $rng_name {
            a: $vector,
            b: $vector,
            c: $vector,
            d: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                let e = self.a - self.b.rotate_left($x);
                self.a = self.b ^ rotate_left!(self.c, $z, $vector);
                self.b = self.c + self.d;
                self.c = self.d + e;
                self.d = e + self.a;
                self.d
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 4];
                rng.try_fill(seed.as_byte_slice_mut())?;

                let a = seed[0];
                let b = seed[1];
                let c = seed[2];
                let mut d = seed[3];

                let eq = |x: $vector, n| x.eq($vector::splat(n));
                let ne = |x: $vector, n| x.ne($vector::splat(n));

                // PractRand: block the cycles of length 1
                let cmp = eq(d & 0x80093300, 0);
                d += 1 & $vector::from_bits(cmp & ne(a, 0) & ne(b, 0) & ne(c, 0) & ne(d, 0));
                d += 1 & $vector::from_bits(
                    cmp
                        & eq(a, 0x77777777)
                        & eq(b, 0x55555555)
                        & eq(c, 0x11111111)
                        & eq(d, 0x44444444),
                );
                d += 1 & $vector::from_bits(
                    cmp
                        & eq(a, 0x5591F2E3)
                        & eq(b, 0x69EBA6CD)
                        & eq(c, 0x2A171E3D)
                        & eq(d, 0x3FD48890),
                );
                d += 1 & $vector::from_bits(
                    cmp
                        & eq(a, 0x47CB8D56)
                        & eq(b, 0xAE9B35A7)
                        & eq(c, 0x5C78F4A8)
                        & eq(d, 0x522240FF),
                );

                Ok(Self { a, b, c, d })
            }
        }
    };

    // Other sets that achieve 8.8 bits of avalanche include (9,16), (9,24),
    // (10,16), (10,24), (11,16), (11,24), (25,8), (25,16), (26,8), (26,16),
    // (26,17), and (27,16).

    (32bit: $rng_name:ident, $vector:ident) => {
        // make_jsf!($rng_name, $vector, 27, 17);
        make_jsf!($rng_name, $vector, 9, 16);
    };

    (64bit: $rng_name:ident, $vector:ident) => {
        make_jsf!($rng_name, $vector, 39, 11);
    };
}

// (where `l` is stream length)
// (using average cycle length)
// (multiple parameters could be used, though slow on older hardware)
// Listing probability of overlap somewhere:                Probability

make_jsf! { 32bit: Jsf32x2, u32x2 } // 2^2 * l / 2^127 ≈    l * 2^-125
make_jsf! { 32bit: Jsf32x4, u32x4 } // 4^2 * l / 2^127 ≈    l * 2^-123
make_jsf! { 32bit: Jsf32x8, u32x8 } // 8^2 * l / 2^127 ≈    l * 2^-121
make_jsf! { 32bit: Jsf32x16, u32x16 } // 16^2 * l / 2^127 ≈ l * 2^-119

make_jsf! { 64bit: Jsf64x2, u64x2 } // 2^2 * l / 2^255 ≈    l * 2^-253
make_jsf! { 64bit: Jsf64x4, u64x4 } // 4^2 * l / 2^255 ≈    l * 2^-251
make_jsf! { 64bit: Jsf64x8, u64x8 } // 8^2 * l / 2^255 ≈    l * 2^-249
