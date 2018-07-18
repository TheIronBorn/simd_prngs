//! SFC generators (32-bit).

use std::simd::*;

use rng_impl::*;

macro_rules! rotate_left {
    ($x:expr, 24, u64x2) => {{
        const ROTL_24: [u32; 16] = [3, 4, 5, 6, 7, 0, 1, 2, 11, 12, 13, 14, 15, 8, 9, 10];
        let vec8 = u8x16::from_bits($x);
        let rotated: u8x16 = unsafe { simd_shuffle16(vec8, vec8, ROTL_24) };
        u64x2::from_bits(rotated)
    }};
    ($x:expr, 24, u64x4) => {{
        const ROTL_24: [u32; 32] = [3, 4, 5, 6, 7, 0, 1, 2, 11, 12, 13, 14, 15, 8, 9, 10, 19, 20, 21, 22, 23, 16, 17, 18, 27, 28, 29, 30, 31, 24, 25, 26];
        let vec8 = u8x32::from_bits($x);
        let rotated: u8x32 = unsafe { simd_shuffle32(vec8, vec8, ROTL_24) };
        u64x4::from_bits(rotated)
    }};
    ($x:expr, 24, u64x8) => {{
        const ROTL_24: [u32; 64] = [3, 4, 5, 6, 7, 0, 1, 2, 11, 12, 13, 14, 15, 8, 9, 10, 19, 20, 21, 22, 23, 16, 17, 18, 27, 28, 29, 30, 31, 24, 25, 26, 35, 36, 37, 38, 39, 32, 33, 34, 43, 44, 45, 46, 47, 40, 41, 42, 51, 52, 53, 54, 55, 48, 49, 50, 59, 60, 61, 62, 63, 56, 57, 58];
        let vec8 = u8x64::from_bits($x);
        let rotated: u8x64 = unsafe { simd_shuffle64(vec8, vec8, ROTL_24) };
        u64x8::from_bits(rotated)
    }};
    ($x:expr, $rot:expr, $y:ident) => {{
        $x.rotate_left($rot)
    }};
}

macro_rules! make_sfc_simd {
    ($rng_name:ident, $vector:ident, $rot:expr, $shr:expr, $shl:expr) => {
        /// An SIMD implementation of Chris Doty-Humphrey's Small Fast Counting RNG
        ///
        /// - Author: Chris Doty-Humphrey
        /// - License: Public domain
        /// - Source: [PractRand](http://pracrand.sourceforge.net/)
        /// - Passes BigCrush and PractRand
        pub struct $rng_name {
            a: $vector,
            b: $vector,
            c: $vector,
            counter: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                let tmp = self.a + self.b + self.counter;
                self.counter += 1;
                self.a = self.b ^ (self.b >> $shr);
                self.b = self.c + (self.c << $shl);
                self.c = rotate_left!(self.c, $rot, $vector) + tmp;
                tmp
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!();
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 3];
                rng.try_fill(seed.as_byte_slice_mut())?;

                Ok(Self {
                    a: seed[0],
                    b: seed[1],
                    c: seed[2],
                    counter: $vector::splat(1),
                })
            }
        }
    };

    ( 64bit: $rng_name:ident, $vector:ident ) => {
        make_sfc_simd! { $rng_name, $vector, 24, 11, 3 }
    };
    ( 32bit: $rng_name:ident, $vector:ident ) => {
        make_sfc_simd! { $rng_name, $vector, 21, 9, 3 }
    };
    ( 16bit: $rng_name:ident, $vector:ident ) => {
        make_sfc_simd! { $rng_name, $vector, 6, 5, 3 }
    };
}

// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (some counter-based techniques could be adapted)
// Listing probability of overlap somewhere:                     Probability

make_sfc_simd! { 64bit: Sfc64x2, u64x2 } // 2^2 * l / 2^255 ≈    l * 2^-253
make_sfc_simd! { 64bit: Sfc64x4, u64x4 } // 4^2 * l / 2^255 ≈    l * 2^-251
make_sfc_simd! { 64bit: Sfc64x8, u64x8 } // 8^2 * l / 2^255 ≈    l * 2^-249

make_sfc_simd! { 32bit: Sfc32x2, u32x2 } // 2^2 * l / 2^128 ≈    l * 2^-126
make_sfc_simd! { 32bit: Sfc32x4, u32x4 } // 4^2 * l / 2^128 ≈    l * 2^-124
make_sfc_simd! { 32bit: Sfc32x8, u32x8 } // 8^2 * l / 2^128 ≈    l * 2^-122
make_sfc_simd! { 32bit: Sfc32x16, u32x16 } // 16^2 * l / 2^128 ≈ l * 2^-120

make_sfc_simd! { 16bit: Sfc16x2, u16x2 } // 2^2 * l / 2^63 ≈     l * 2^-61
make_sfc_simd! { 16bit: Sfc16x4, u16x4 } // 4^2 * l / 2^63 ≈     l * 2^-59
make_sfc_simd! { 16bit: Sfc16x8, u16x8 } // 8^2 * l / 2^63 ≈     l * 2^-57
make_sfc_simd! { 16bit: Sfc16x16, u16x16 } // 16^2 * l / 2^63 ≈  l * 2^-55
make_sfc_simd! { 16bit: Sfc16x32, u16x32 } // 32^2 * l / 2^63 ≈  l * 2^-52
