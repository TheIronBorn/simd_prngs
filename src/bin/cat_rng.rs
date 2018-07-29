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

extern crate packed_simd;
extern crate rand;
extern crate simd_prngs;

use packed_simd::*;
use std::io::{self, BufWriter};
use std::io::prelude::*;

use rand::FromEntropy;
use simd_prngs::*;

// allows `{from,write_to}_slice_aligned`
#[repr(align(16))]
struct Aligned<T>(T);

// Change these to test a different PRNG
type SimdRng = Ars5;
type Vec8 = u8x16;

#[inline(always)]
fn fill_bytes_via_simd(rng: &mut SimdRng, dest: &mut Aligned<[u8; 64]>) {
    const CHUNK_SIZE: usize = Vec8::lanes();
    assert_eq!(dest.0.len() % CHUNK_SIZE, 0);

    let mut read_len = 0;
    for _ in 0..dest.0.len() / CHUNK_SIZE {
        let mut results = Vec8::from_bits(rng.generate());
        results.write_to_slice_aligned(&mut dest.0[read_len..read_len + CHUNK_SIZE]);
        read_len += CHUNK_SIZE;
    }
}

fn main() -> io::Result<()> {
    let mut rng = SimdRng::from_entropy();

    let mut buf = Aligned([0u8; 64]);
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut writer = BufWriter::new(lock);

    loop {
        fill_bytes_via_simd(&mut rng, &mut buf);
        writer.write_all(&buf.0)?;
    }
}
