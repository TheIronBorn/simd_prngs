// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A small utility to concatenate the output of an RNG to stdout.
//!
//! `$ cat_rng | RNG_test stdin -multithreaded`
//! NOTE: `stdin32` seems to be fastest, I'm not sure what sort of effect it
//! might have on test strength.
//!
//! `-tlmin 256GB` might also be useful

#![feature(stdsimd)]

extern crate rand;
extern crate simd_prngs;

use std::simd::*;
use std::io::{self, BufWriter};
use std::io::prelude::*;
use std::mem;

use rand::FromEntropy;
use simd_prngs::*;

#[repr(align(16))]
struct Aligned<T>(T);

// Change these to test a different PRNG
type Vector = u32x4;
type SimdRng = Jsf32x4;
type Vec8 = u8x16;

#[inline(always)]
fn fill_bytes_via_simd(rng: &mut SimdRng, dest: &mut [u8]) {
    // Forced inlining will keep the result in SIMD registers if
    // the code using it also uses it in a SIMD context.
    const CHUNK_SIZE: usize = mem::size_of::<Vector>();
    let mut read_len = 0;
    for _ in 0..dest.len() / CHUNK_SIZE {
        let mut results = Vec8::from_bits(rng.generate());
        // results = results.to_le(); // TODO: look into reverse store intrinsics
        results.store_aligned(&mut dest[read_len..read_len + CHUNK_SIZE]);
        read_len += CHUNK_SIZE;
    }
    let remainder = dest.len() % CHUNK_SIZE;
    if remainder > 0 {
        // This could be `ptr::copy_nonoverlapping` which doubles
        // the speed (for non-SIMD contexts), but I'm not sure SIMD is happy
        // with it.
        let results = Vec8::from_bits(rng.generate());
        // results = results.to_le();
        let len = dest.len() - remainder;

        // ensure `store_aligned` safety, perhaps overzealous
        let mut buf = Aligned([0_u8; Vec8::lanes()]);
        results.store_aligned(&mut buf.0);

        dest[len..].copy_from_slice(&buf.0[..remainder]);
    }
}

fn main() -> io::Result<()> {
    let mut rng = SimdRng::from_entropy();

    let mut buf = Aligned([0u8; 64]);
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut writer = BufWriter::new(lock);

    loop {
        fill_bytes_via_simd(&mut rng, &mut buf.0);
        writer.write_all(&buf.0)?;
    }
}
