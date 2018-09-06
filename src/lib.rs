//! A crate researching various SIMD PRNG speeds.
//!
//! You need nightly Rust and SIMD capable hardware to use this crate.
//!
//! To use it, run:
//! ```console
//! $ RUSTFLAGS='-C target-cpu=native' cargo bench
//! ```

#![cfg_attr(
    feature = "cargo-clippy",
    allow(unreadable_literal, reverse_range_loop, cast_lossless)
)]

extern crate packed_simd;
extern crate rand;
extern crate rand_core;

use packed_simd::*;
use std::{mem, slice};

#[macro_use]
mod macros;
mod prngs;
pub use prngs::*;

mod rng_impl {
    pub use packed_simd::*;
    pub use rand::{Error, Rng, RngCore, SeedableRng};
    pub use AsByteSliceMut;
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
