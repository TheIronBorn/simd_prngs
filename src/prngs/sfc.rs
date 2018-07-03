// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// https://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! SFC generators (32-bit).

use std::simd::*;

use rng_impl::*;

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
                self.c = self.c.rotate_left($rot) + tmp;
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
