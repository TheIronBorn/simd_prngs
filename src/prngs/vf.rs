// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// https://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! VeryFast generators.

use std::mem;
use std::simd::*;

use rng_impl::*;

macro_rules! vf_a {
    (
        $rng_name:ident,
        $vector:ident,
        $scalar:ty,
        $half:ident,
        $shuf:ident,
        $indices:expr,
        $rot:expr,
        $shr:expr,
        $shl:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // good speed, 16 bit version fails @ 32 GB, 32 bit version passed 8 TB
                let old = self.a + self.b;
                self.a = self.b ^ (self.b >> $shr);
                self.b = self.c + (self.c << $shl);
                self.c = old + self.c.rotate_left($rot); // $shr,$shl,$rot : 7,3,9 @ 32 bit
                old
            }
        }
    };
}

macro_rules! vf_b {
    ($rng_name:ident, $vector:ident, $scalar:ty, $half:ident, $shuf:ident, $indices:expr, $rot:expr, $shr:expr, $shl:expr) => (
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // best quality: 16 bit fails @ 1 TB, but not as fast ;; switching `a += b ^ c;` for `a ^= b + c;` increases that to 2 TB
                let old = self.a + (self.a << $shl);
                self.a += self.b ^ self.c;
                self.b = self.c ^ (self.c >> $shr);
                self.c = old + self.c.rotate_left($rot);
                old
            }
        }
    )
}

macro_rules! vf_c {
    (
        $rng_name:ident,
        $vector:ident,
        $scalar:ty,
        $half:ident,
        $shuf:ident,
        $indices:expr,
        $rot:expr,
        $shr:expr,
        $shl:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // faster, simpler, lower quality - just 4-6 ops, very few dependent
                // 16 bit: 128 MB, 32 bit: 32 GB
                let old = self.a + self.b;
                self.a = self.b + self.c.rotate_left($rot);
                self.b = self.c + (self.c << $shl);
                self.c = old;
                self.c
            }
        }
    };
}

macro_rules! vf_d {
    (
        $rng_name:ident,
        $vector:ident,
        $scalar:ty,
        $half:ident,
        $shuf:ident,
        $indices:expr,
        $rot:expr,
        $shr:expr,
        $shl:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // another alternative
                // 16 bit: 1 GB, 32 bit: 2 TB
                let old = self.a + self.b;
                self.a = self.b;
                self.b = self.c + (self.c << $shl);
                self.c = self.c.rotate_left($rot);
                self.a += self.c;
                self.c += old;
                self.a
            }
        }
    };
}

macro_rules! vf_e {
    (
        $rng_name:ident,
        $vector:ident,
        $scalar:ty,
        $half:ident,
        $shuf:ident,
        $indices:expr,
        $rot:expr,
        $shr:expr,
        $shl:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                /// rotate by half, using vector shuffles (faster on older hardware)
                #[inline(always)]
                fn rotate_half(x: $vector) -> $vector {
                    let x = $half::from_bits(x);
                    let r: $half = unsafe { $shuf(x, x, $indices) };
                    $vector::from_bits(r)
                }

                // uses multiplication, only 2 words, but pretty good aside from that:
                //16: 1 GB, 32 bit: > 32 TB
                #[allow(overflowing_literals)]
                let old = self.a * 0x92ec64765925a395;
                self.a = self.b ^ rotate_half(self.a);
                self.b = old;
                self.a + self.b
            }
        }
    };
}

macro_rules! vf_f {
    (
        $rng_name:ident,
        $vector:ident,
        $scalar:ty,
        $half:ident,
        $shuf:ident,
        $indices:expr,
        $rot:expr,
        $shr:expr,
        $shl:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                /// rotate by half, using vector shuffles (faster on older hardware)
                #[inline(always)]
                fn rotate_half(x: $vector) -> $vector {
                    let x = $half::from_bits(x);
                    let r: $half = unsafe { $shuf(x, x, $indices) };
                    $vector::from_bits(r)
                }

                #[allow(overflowing_literals)]
                let old = self.a * 0x92ec64765925a395;
                self.c += 1;
                self.a = rotate_half(self.a) ^ self.b ^ self.c;
                self.b = old;
                self.a
            }
        }
    };
}

macro_rules! vf_g {
    (
        $rng_name:ident,
        $vector:ident,
        $scalar:ty,
        $half:ident,
        $shuf:ident,
        $indices:expr,
        $rot:expr,
        $shr:expr,
        $shl:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                const BITS: $scalar = mem::size_of::<$scalar>() as $scalar * 8;

                let old = self.a ^ (self.a >> (BITS / 2));
                //self.c += (self.c << 3) + 1;
                self.a += self.b + (self.b << 3);
                self.c += 1;
                self.b ^= old + self.c;
                self.a
            }
        }
    };
}

macro_rules! make_vf {
    ($rng_name:ident, $version:ident, $vector:ident, $scalar:ty, $half:ident, $shuf:ident, $indices:expr, $rot:expr, $shr:expr, $shl:expr) => {
        pub struct $rng_name {
            a: $vector,
            b: $vector,
            #[allow(dead_code)]
            c: $vector,
        }

        $version!($rng_name, $vector, $scalar, $half, $shuf, $indices, $rot, $shr, $shl);

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
                })
            }
        }
    };


    ( versions: $vector:ident, $scalar:ty, $half:ident, $shuf:ident, $indices:expr, $rot:expr, $shr:expr, $shl:expr,
        $name_a:ident, $name_b:ident, $name_c:ident, $name_d:ident, $name_e:ident, $name_f:ident, $name_g:ident,
    ) => {
        make_vf! { $name_a, vf_a, $vector, $scalar, $half, $shuf, $indices, $rot, $shr, $shl }
        make_vf! { $name_b, vf_b, $vector, $scalar, $half, $shuf, $indices, $rot, $shr, $shl }
        make_vf! { $name_c, vf_c, $vector, $scalar, $half, $shuf, $indices, $rot, $shr, $shl }
        make_vf! { $name_d, vf_d, $vector, $scalar, $half, $shuf, $indices, $rot, $shr, $shl }
        make_vf! { $name_e, vf_e, $vector, $scalar, $half, $shuf, $indices, $rot, $shr, $shl }
        make_vf! { $name_f, vf_f, $vector, $scalar, $half, $shuf, $indices, $rot, $shr, $shl }
        make_vf! { $name_g, vf_g, $vector, $scalar, $half, $shuf, $indices, $rot, $shr, $shl }
    };

    ( 64bit: $vector:ident, $half:ident, $indices:expr, $shuf:ident, $($rng_name:ident,)+) => {
        make_vf! { versions: $vector, u64, $half, $shuf, $indices, 29, 9, 3, $($rng_name,)+ }
    };
    ( 32bit: $vector:ident, $half:ident, $indices:expr, $shuf:ident, $($rng_name:ident,)+) => {
        make_vf! { versions: $vector, u32, $half, $shuf, $indices, 13, 7, 3, $($rng_name,)+ }
    };
    ( 16bit: $vector:ident, $half:ident, $indices:expr, $shuf:ident, $($rng_name:ident,)+) => {
        make_vf! { versions: $vector, u16, $half, $shuf, $indices, 7, 3, 2, $($rng_name,)+ }
    };
    /*( 8bit: $vector:ident, $($rng_name:ident,)+) => {
        make_vf! { versions: $vector, u8, 3, 2, 2, $($rng_name,)+ }
    };*/
}

// WARNING: must be in proper order

make_vf! { 64bit: u64x2, u32x4, [1, 0, 3, 2], simd_shuffle4, VeryFast64x2a, VeryFast64x2b, VeryFast64x2c, VeryFast64x2d, VeryFast64x2e, VeryFast64x2f, VeryFast64x2g, }
make_vf! { 64bit: u64x4, u32x8, [1, 0, 3, 2, 5, 4, 7, 6], simd_shuffle8, VeryFast64x4a, VeryFast64x4b, VeryFast64x4c, VeryFast64x4d, VeryFast64x4e, VeryFast64x4f, VeryFast64x4g, }
make_vf! { 64bit: u64x8, u32x16, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14], simd_shuffle16, VeryFast64x8a, VeryFast64x8b, VeryFast64x8c, VeryFast64x8d, VeryFast64x8e, VeryFast64x8f, VeryFast64x8g, }

make_vf! { 32bit: u32x2, u16x4, [1, 0, 3, 2], simd_shuffle4, VeryFast32x2a, VeryFast32x2b, VeryFast32x2c, VeryFast32x2d, VeryFast32x2e, VeryFast32x2f, VeryFast32x2g, }
make_vf! { 32bit: u32x4, u16x8, [1, 0, 3, 2, 5, 4, 7, 6], simd_shuffle8, VeryFast32x4a, VeryFast32x4b, VeryFast32x4c, VeryFast32x4d, VeryFast32x4e, VeryFast32x4f, VeryFast32x4g, }
make_vf! { 32bit: u32x8, u16x16, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14], simd_shuffle16, VeryFast32x8a, VeryFast32x8b, VeryFast32x8c, VeryFast32x8d, VeryFast32x8e, VeryFast32x8f, VeryFast32x8g, }
make_vf! { 32bit: u32x16, u16x32, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30], simd_shuffle32, VeryFast32x16a, VeryFast32x16b, VeryFast32x16c, VeryFast32x16d, VeryFast32x16e, VeryFast32x16f, VeryFast32x16g, }

make_vf! { 16bit: u16x2, u8x4, [1, 0, 3, 2], simd_shuffle4, VeryFast16x2a, VeryFast16x2b, VeryFast16x2c, VeryFast16x2d, VeryFast16x2e, VeryFast16x2f, VeryFast16x2g, }
make_vf! { 16bit: u16x4, u8x8, [1, 0, 3, 2, 5, 4, 7, 6], simd_shuffle8, VeryFast16x4a, VeryFast16x4b, VeryFast16x4c, VeryFast16x4d, VeryFast16x4e, VeryFast16x4f, VeryFast16x4g, }
make_vf! { 16bit: u16x8, u8x16, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14], simd_shuffle16, VeryFast16x8a, VeryFast16x8b, VeryFast16x8c, VeryFast16x8d, VeryFast16x8e, VeryFast16x8f, VeryFast16x8g, }
make_vf! { 16bit: u16x16, u8x32, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30], simd_shuffle32, VeryFast16x16a, VeryFast16x16b, VeryFast16x16c, VeryFast16x16d, VeryFast16x16e, VeryFast16x16f, VeryFast16x16g, }
make_vf! { 16bit: u16x32, u8x64,
    [1,0,3,2,5,4,7,6,9,8,11,10,13,12,15,14,17,16,19,18,21,20,23,22,25,24,27,26,29,28,31,30,33,32,35,34,37,36,39,38,41,40,43,42,45,44,47,46,49,48,51,50,53,52,55,54,57,56,59,58,61,60,63,62],
    simd_shuffle64,
    VeryFast16x32a, VeryFast16x32b, VeryFast16x32c, VeryFast16x32d, VeryFast16x32e, VeryFast16x32f, VeryFast16x32g,
}

/*make_vf! { 8bit: u8x2, VeryFast8x2a, VeryFast8x2b, VeryFast8x2c, VeryFast8x2d, VeryFast8x2e, VeryFast8x2f, VeryFast8x2g, }
make_vf! { 8bit: u8x4, VeryFast8x4a, VeryFast8x4b, VeryFast8x4c, VeryFast8x4d, VeryFast8x4e, VeryFast8x4f, VeryFast8x4g, }
make_vf! { 8bit: u8x8, VeryFast8x8a, VeryFast8x8b, VeryFast8x8c, VeryFast8x8d, VeryFast8x8e, VeryFast8x8f, VeryFast8x8g, }
make_vf! { 8bit: u8x16, VeryFast8x16a, VeryFast8x16b, VeryFast8x16c, VeryFast8x16d, VeryFast8x16e, VeryFast8x16f, VeryFast8x16g, }
make_vf! { 8bit: u8x32, VeryFast8x32a, VeryFast8x32b, VeryFast8x32c, VeryFast8x32d, VeryFast8x32e, VeryFast8x32f, VeryFast8x32g, }
make_vf! { 8bit: u8x64, VeryFast8x64a, VeryFast8x64b, VeryFast8x64c, VeryFast8x64d, VeryFast8x64e, VeryFast8x64f, VeryFast8x64g, }*/
