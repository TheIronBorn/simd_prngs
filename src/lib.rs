//! A crate researching various SIMD PRNG speeds.
//!
//! You need nightly Rust and SIMD capable hardware to use this crate.
//!
//! To use it, run:
//! ```console
//! $ RUSTFLAGS='-C target-cpu=native' cargo bench
//! ```

#![feature(stdsimd)]
#![feature(platform_intrinsics)]

extern crate rand;

use std::simd::*;
use std::{mem, slice};

mod prngs;
pub use prngs::*;

mod rng_impl {
    pub use rand::{Error, Rng, RngCore, SeedableRng};
    pub use shuffles::*;
    pub use {AsByteSliceMut, Rotates};
}

/// Trait for casting types to byte slices.
pub trait AsByteSliceMut {
    /// Return a mutable reference to self as a byte slice
    fn as_byte_slice_mut(&mut self) -> &mut [u8];
}

impl AsByteSliceMut for [u8] {
    #[inline]
    fn as_byte_slice_mut(&mut self) -> &mut [u8] {
        self
    }
}

macro_rules! impl_as_byte_slice_simd {
    ($($t:ty,)+) => (
        $(
            impl AsByteSliceMut for [$t] {
                #[inline]
                fn as_byte_slice_mut(&mut self) -> &mut [u8] {
                    unsafe {
                        slice::from_raw_parts_mut(&mut self[0]
                            as *mut $t
                            as *mut u8,
                            self.len() * mem::size_of::<$t>()
                        )
                    }
                }
            }
        )+
    )
}

impl_as_byte_slice_simd! {
    u8x2,  u8x4,  u8x8,  u8x16,  u8x32,  u8x64,
    u16x2, u16x4, u16x8, u16x16, u16x32,
    u32x2, u32x4, u32x8, u32x16,
    u64x2, u64x4, u64x8,
}

pub trait Rotates<T> {
    fn rotate_left(self, n: T) -> Self;
    fn rotate_right(self, n: T) -> Self;
}

macro_rules! impl_rotates {
    ($elem_ty:ty, $($ty:ty,)+) => {
        $(
            impl Rotates<$elem_ty> for $ty {
                #[inline(always)]
                fn rotate_left(self, n: $elem_ty) -> Self {
                    const BITS: $elem_ty = mem::size_of::<$elem_ty>() as $elem_ty * 8;
                    // Protect against undefined behavior for over-long bit shifts
                    let n = n % BITS;
                    (self << n) | (self >> ((BITS - n) % BITS))
                }

                #[inline(always)]
                fn rotate_right(self, n: $elem_ty) -> Self {
                    const BITS: $elem_ty = mem::size_of::<$elem_ty>() as $elem_ty * 8;
                    // Protect against undefined behavior for over-long bit shifts
                    let n = n % BITS;
                    (self >> n) | (self << ((BITS - n) % BITS))
                }
            }
        )+
    };
}

impl_rotates! { u8, u8x2, u8x4, u8x8, u8x16, u8x32, u8x64, }
impl_rotates! { u16, u16x2, u16x4, u16x8, u16x16, u16x32, }
impl_rotates! { u32, u32x2, u32x4, u32x8, u32x16, }
impl_rotates! { u64, u64x2, u64x4, u64x8, }

#[allow(dead_code)]
mod shuffles {
    extern "platform-intrinsic" {
        pub fn simd_shuffle2<T, U>(a: T, b: T, indices: [u32; 2]) -> U;
        pub fn simd_shuffle4<T, U>(a: T, b: T, indices: [u32; 4]) -> U;
        pub fn simd_shuffle8<T, U>(a: T, b: T, indices: [u32; 8]) -> U;
        pub fn simd_shuffle16<T, U>(a: T, b: T, indices: [u32; 16]) -> U;
        pub fn simd_shuffle32<T, U>(a: T, b: T, indices: [u32; 32]) -> U;
        pub fn simd_shuffle64<T, U>(a: T, b: T, indices: [u32; 64]) -> U;
    }
}
