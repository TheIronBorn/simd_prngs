// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// https://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! SFC Alternate generators.

use rng_impl::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! sfc_alt_a {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //experiment with larger pseudo-counter
                self.counter += 1;
                // counter2 += counter + (counter ? 0 : 1);//2-word LCG
                let cmp = self.counter.eq($vector::splat(0));
                self.counter2 += self.counter - $vector::from_bits(cmp);
                let tmp = self.a + self.b; //counter2;
                //a = b ^ (b >> $sh2);
                //a = b + (b << $sh3);
                self.a = self.b + self.counter2;
                self.b = rotate_left!(self.b, $sh1, $vector) + tmp;
                self.a
            }
        }
    };
}

macro_rules! sfc_alt_b {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //SFC 3:
                let tmp = self.a + self.b + self.counter;
                self.counter += 1;
                self.a = self.b ^ (self.b >> $sh2);
                self.b = rotate_left!(self.b, $sh1, $vector) + tmp;
                tmp
            }
        }
    };
}

macro_rules! sfc_alt_c {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //SFC 4, 16 bit version >8 TB (64 GB w/o counter)
                let old = self.a + self.b + self.counter; //64 GB on counter, 8 TB on b
                self.counter += 1;
                self.a = self.b ^ (self.b >> $sh2); //128 GB?
                self.b = self.c + (self.c << $sh3); //1 TB
                self.c = old + rotate_left!(self.c, $sh1, $vector); //important!
                old
            }
        }
    };
}

macro_rules! sfc_alt_d {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //okay speed, 16 bit version >2 TB (256 GB w/o counter), 32 bit @ ?
                let old = self.a + (self.a << $sh3);
                self.a = self.b + self.c + self.counter;
                self.counter += 1;
                self.b = self.c ^ (self.c >> $sh2);
                self.c = rotate_left!(self.c, $sh1, $vector) + old;
                old
            }
        }
    };
}

macro_rules! sfc_alt_e {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //too slow, 16 bit version ??? (4 TB w/o counter)
                let old = self.a + self.b + self.counter;
                self.counter += 1;
                self.a = old ^ rotate_left!(self.a, $sh2, $vector);
                self.b = self.c + (self.c << $sh3);
                self.c = old + rotate_left!(self.c, $sh1, $vector);
                old
            }
        }
    };
}

macro_rules! sfc_alt_f {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //too slow, 16 bit version ??? (2 TB w/o counter)
                let old = self.a + (self.a << $sh3);
                self.a += self.b ^ self.c;
                self.b = self.c ^ (self.c >> $sh2) ^ self.counter;
                self.counter += 1;
                self.c = old + rotate_left!(self.c, $sh1, $vector);
                old
            }
        }
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! sfc_alt_g {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //faster, 16 bit version failed @ 64-128 GB (4 GB w/o counter), 32 bit @ ? (passed 16 TB w/o counter)
                let old = self.a + self.b;
                self.a = self.b + self.counter;
                self.counter += 1;
                self.b = self.c ^ (self.c >> $sh2);
                self.c = old + rotate_left!(self.c, $sh1, $vector);
                old
            }
        }
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! sfc_alt_h {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //good speed, 16 bit version failed @ >512 GB (32 GB w/o counter), 32 bit @ ? (? w/o counter)
                let old = self.a + self.b + self.counter;
                self.counter += 1;
                self.a = self.b + (self.b << $sh3);
                self.b = self.c ^ (self.c >> $sh2);
                self.c = old + rotate_left!(self.c, $sh1, $vector);
                old
            }
        }
    };
}

macro_rules! sfc_alt_i {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //???
                let old = self.a + self.counter;
                self.counter += 1;
                self.a = rotate_left!(self.a, 3, $vector) ^ (self.a + self.b);
                self.b = rotate_left!(self.b, 7, $vector) ^ (self.b + self.c);
                self.c = rotate_left!(self.c, 11, $vector) ^ (self.c + old);
                old ^ self.b
            }
        }
    };
}

macro_rules! sfc_alt_j {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // Some rotates here are larger than 8, which means this won't
                // work for 8-bit Sfc

                self.a += rotate_left!(self.a, 7, $vector);
                self.b = rotate_left!(self.b, 13, $vector) + self.b + (self.b << 3);
                self.c = (self.c + (self.c << 7)) ^ rotate_left!(self.c, 11, $vector);
                self.a ^ self.b ^ self.c
            }
        }
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! sfc_alt_k {
    (
        $rng_name:ident,
        $vector:ident,
        constants: $sh1:expr,
        $sh2:expr,
        $sh3:expr,
        e1: $e_sh:expr,
        e2: $e_sh1:expr,
        $e_sh2:expr
    ) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // My testing puts it at >512GB (64-bit version)

                //VERY good speed, 16 bit version failed @ 256 GB (2 GB w/o counter), 32 bit @ ?
                self.a += self.b; self.b -= self.c;
                self.c += self.a; self.a ^= self.counter;
                self.counter += 1;
                self.c = rotate_left!(self.c, $e_sh, $vector);

                // This variant seems to be missing a line like the `l` variant:
                // `self.b += self.b << $e_sh2;`

                self.a
            }
        }
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! sfc_alt_l {
    ($rng_name:ident, $vector:ident, constants: $sh1:expr, $sh2:expr, $sh3:expr, e1: $e_sh:expr, e2: $e_sh1:expr, $e_sh2:expr) => {
        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                //VERY good speed, 16 bit version failed @ 16 TB (1 TB w/o counter), 32 bit @ > 4 TB w/o counter
                self.a += self.b; self.b -= self.c;
                self.c += self.a; self.a ^= self.counter;
                self.counter += 1;
                // with 64-bit, `$e_sh1` is 48 which is divisible by 8. We could
                // then implement this rotate with a vector shuffle (might be
                // faster on older hardware)
                self.c = rotate_left!(self.c, $e_sh1, $vector); //cb  with count: ?, 14, 9, ?  ; w/o count: 16, 8, 9, ?
                self.b += self.b << $e_sh2; //ba
                self.a
            }
        }
    };
}

macro_rules! make_sfc {
    ($rng_name:ident, $version:ident, $vector:ident, constants: $sh1:expr, $sh2:expr, $sh3:expr, e1: $e_sh:expr, e2: $e_sh1:expr, $e_sh2:expr) => {
        pub struct $rng_name {
            a: $vector,
            b: $vector,
            #[allow(dead_code)]
            c: $vector,
            #[allow(dead_code)]
            counter: $vector,
            #[allow(dead_code)]
            counter2: $vector,
        }

        $version!($rng_name, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2);

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
                    counter: $vector::splat(0),
                    counter2: $vector::splat(1),
                })
            }
        }
    };


    ( versions: $vector:ident, constants: $sh1:expr, $sh2:expr, $sh3:expr, e1: $e_sh:expr, e2: $e_sh1:expr, $e_sh2:expr,
        $name_a:ident, $name_b:ident, $name_c:ident, $name_d:ident, $name_e:ident, $name_f:ident,
        $name_g:ident, $name_h:ident, $name_i:ident, $name_j:ident, $name_k:ident, $name_l:ident,
    ) => {
        make_sfc! { $name_a, sfc_alt_a, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_b, sfc_alt_b, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_c, sfc_alt_c, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_d, sfc_alt_d, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_e, sfc_alt_e, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_f, sfc_alt_f, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_g, sfc_alt_g, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_h, sfc_alt_h, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_i, sfc_alt_i, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_j, sfc_alt_j, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_k, sfc_alt_k, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
        make_sfc! { $name_l, sfc_alt_l, $vector, constants: $sh1, $sh2, $sh3, e1: $e_sh, e2: $e_sh1, $e_sh2 }
    };

    ( 64bit: $vector:ident, $($rng_name:ident,)+) => {
        make_sfc! { versions: $vector, constants: 25, 12, 3, e1: 43, e2: 48, 3, $($rng_name,)+ }
    };
    ( 32bit: $vector:ident, $($rng_name:ident,)+) => {
        make_sfc! { versions: $vector, constants: 25, 8, 3, e1: 23, e2: 14, 3, $($rng_name,)+ }
    };
    ( 16bit: $vector:ident, $($rng_name:ident,)+) => {
        make_sfc! { versions: $vector, constants: 7, 3, 2, e1: 11, e2: 9, 3, $($rng_name,)+ }
    };
    ( 8bit: $vector:ident, $($rng_name:ident,)+) => {
        make_sfc! { versions: $vector, constants: 3, 2, 1, e1: 0, e2: 5, 2, $($rng_name,)+ }
    };
}

// WARNING: must be in proper order

make_sfc! { 64bit: u64x2, SfcAlt64x2a, SfcAlt64x2b, SfcAlt64x2c, SfcAlt64x2d, SfcAlt64x2e, SfcAlt64x2f, SfcAlt64x2g, SfcAlt64x2h, SfcAlt64x2i, SfcAlt64x2j, SfcAlt64x2k, SfcAlt64x2l, }
make_sfc! { 64bit: u64x4, SfcAlt64x4a, SfcAlt64x4b, SfcAlt64x4c, SfcAlt64x4d, SfcAlt64x4e, SfcAlt64x4f, SfcAlt64x4g, SfcAlt64x4h, SfcAlt64x4i, SfcAlt64x4j, SfcAlt64x4k, SfcAlt64x4l, }
make_sfc! { 64bit: u64x8, SfcAlt64x8a, SfcAlt64x8b, SfcAlt64x8c, SfcAlt64x8d, SfcAlt64x8e, SfcAlt64x8f, SfcAlt64x8g, SfcAlt64x8h, SfcAlt64x8i, SfcAlt64x8j, SfcAlt64x8k, SfcAlt64x8l, }

make_sfc! { 32bit: u32x2, SfcAlt32x2a, SfcAlt32x2b, SfcAlt32x2c, SfcAlt32x2d, SfcAlt32x2e, SfcAlt32x2f, SfcAlt32x2g, SfcAlt32x2h, SfcAlt32x2i, SfcAlt32x2j, SfcAlt32x2k, SfcAlt32x2l, }
make_sfc! { 32bit: u32x4, SfcAlt32x4a, SfcAlt32x4b, SfcAlt32x4c, SfcAlt32x4d, SfcAlt32x4e, SfcAlt32x4f, SfcAlt32x4g, SfcAlt32x4h, SfcAlt32x4i, SfcAlt32x4j, SfcAlt32x4k, SfcAlt32x4l, }
make_sfc! { 32bit: u32x8, SfcAlt32x8a, SfcAlt32x8b, SfcAlt32x8c, SfcAlt32x8d, SfcAlt32x8e, SfcAlt32x8f, SfcAlt32x8g, SfcAlt32x8h, SfcAlt32x8i, SfcAlt32x8j, SfcAlt32x8k, SfcAlt32x8l, }
make_sfc! { 32bit: u32x16, SfcAlt32x16a, SfcAlt32x16b, SfcAlt32x16c, SfcAlt32x16d, SfcAlt32x16e, SfcAlt32x16f, SfcAlt32x16g, SfcAlt32x16h, SfcAlt32x16i, SfcAlt32x16j, SfcAlt32x16k, SfcAlt32x16l, }

make_sfc! { 16bit: u16x2, SfcAlt16x2a, SfcAlt16x2b, SfcAlt16x2c, SfcAlt16x2d, SfcAlt16x2e, SfcAlt16x2f, SfcAlt16x2g, SfcAlt16x2h, SfcAlt16x2i, SfcAlt16x2j, SfcAlt16x2k, SfcAlt16x2l, }
make_sfc! { 16bit: u16x4, SfcAlt16x4a, SfcAlt16x4b, SfcAlt16x4c, SfcAlt16x4d, SfcAlt16x4e, SfcAlt16x4f, SfcAlt16x4g, SfcAlt16x4h, SfcAlt16x4i, SfcAlt16x4j, SfcAlt16x4k, SfcAlt16x4l, }
make_sfc! { 16bit: u16x8, SfcAlt16x8a, SfcAlt16x8b, SfcAlt16x8c, SfcAlt16x8d, SfcAlt16x8e, SfcAlt16x8f, SfcAlt16x8g, SfcAlt16x8h, SfcAlt16x8i, SfcAlt16x8j, SfcAlt16x8k, SfcAlt16x8l, }
make_sfc! { 16bit: u16x16, SfcAlt16x16a, SfcAlt16x16b, SfcAlt16x16c, SfcAlt16x16d, SfcAlt16x16e, SfcAlt16x16f, SfcAlt16x16g, SfcAlt16x16h, SfcAlt16x16i, SfcAlt16x16j, SfcAlt16x16k, SfcAlt16x16l, }
make_sfc! { 16bit: u16x32, SfcAlt16x32a, SfcAlt16x32b, SfcAlt16x32c, SfcAlt16x32d, SfcAlt16x32e, SfcAlt16x32f, SfcAlt16x32g, SfcAlt16x32h, SfcAlt16x32i, SfcAlt16x32j, SfcAlt16x32k, SfcAlt16x32l, }

make_sfc! { 8bit: u8x2, SfcAlt8x2a, SfcAlt8x2b, SfcAlt8x2c, SfcAlt8x2d, SfcAlt8x2e, SfcAlt8x2f, SfcAlt8x2g, SfcAlt8x2h, SfcAlt8x2i, SfcAlt8x2j, SfcAlt8x2k, SfcAlt8x2l, }
make_sfc! { 8bit: u8x4, SfcAlt8x4a, SfcAlt8x4b, SfcAlt8x4c, SfcAlt8x4d, SfcAlt8x4e, SfcAlt8x4f, SfcAlt8x4g, SfcAlt8x4h, SfcAlt8x4i, SfcAlt8x4j, SfcAlt8x4k, SfcAlt8x4l, }
make_sfc! { 8bit: u8x8, SfcAlt8x8a, SfcAlt8x8b, SfcAlt8x8c, SfcAlt8x8d, SfcAlt8x8e, SfcAlt8x8f, SfcAlt8x8g, SfcAlt8x8h, SfcAlt8x8i, SfcAlt8x8j, SfcAlt8x8k, SfcAlt8x8l, }
make_sfc! { 8bit: u8x16, SfcAlt8x16a, SfcAlt8x16b, SfcAlt8x16c, SfcAlt8x16d, SfcAlt8x16e, SfcAlt8x16f, SfcAlt8x16g, SfcAlt8x16h, SfcAlt8x16i, SfcAlt8x16j, SfcAlt8x16k, SfcAlt8x16l, }
make_sfc! { 8bit: u8x32, SfcAlt8x32a, SfcAlt8x32b, SfcAlt8x32c, SfcAlt8x32d, SfcAlt8x32e, SfcAlt8x32f, SfcAlt8x32g, SfcAlt8x32h, SfcAlt8x32i, SfcAlt8x32j, SfcAlt8x32k, SfcAlt8x32l, }
make_sfc! { 8bit: u8x64, SfcAlt8x64a, SfcAlt8x64b, SfcAlt8x64c, SfcAlt8x64d, SfcAlt8x64e, SfcAlt8x64f, SfcAlt8x64g, SfcAlt8x64h, SfcAlt8x64i, SfcAlt8x64j, SfcAlt8x64k, SfcAlt8x64l, }
