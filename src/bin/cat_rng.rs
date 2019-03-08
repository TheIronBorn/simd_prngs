//! A small utility to concatenate the output of an RNG to stdout.
//!
//! `$ cat_rng | RNG_test stdin -multithreaded`

extern crate packed_simd;
extern crate rand;
extern crate simd_prngs;

use std::io;
use std::io::prelude::*;

use rand::prelude::*;

use simd_prngs::*;

// allows `write_to_slice_aligned`
#[repr(align(16))]
struct Aligned<T>(T);

fn main() -> io::Result<()> {
    let seed = rand::rngs::EntropyRng::new().gen();
    // For reproducible test results. Provide readable RNG state?
    println!("{:#?}", seed);

    // Change this to test a different RNG
    let mut rng = Xoroshiro128StarStarX2::from_seed(seed);

    let mut buf = Aligned([0; 4096]);
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    loop {
        rng.fill_bytes_aligned(&mut buf.0);
        writer.write_all(&buf.0)?;
    }
}
