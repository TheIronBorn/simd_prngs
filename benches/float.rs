//! A real-world benchmark of PRNG throughput.

#![feature(test)]

extern crate packed_simd;
extern crate rand;
extern crate simd_prngs;
extern crate test;

use std::mem;
use test::Bencher;

use packed_simd::*;
use rand::distributions::Open01;
use rand::prelude::*;
use simd_prngs::*;

const BENCH_N: u64 = 1 << 10;

macro_rules! make_float_bench {
    ($gen:ident, $uty:ident, $fty:ident) => {
        #[allow(non_snake_case)]
        mod $gen {
            use super::*;

            #[bench]
            fn float(b: &mut Bencher) {
                let mut rng = simd_prngs::$gen::from_rng(thread_rng()).unwrap();

                b.iter(|| {
                    let mut accum = $fty::default();
                    for _ in 0..BENCH_N {
                        let x: $fty = rng.sample(Open01);
                        accum += x;
                    }
                    accum
                });
                b.bytes = BENCH_N * mem::size_of::<$fty>() as u64;
            }
        }
    };
}

for_each_prng! { make_float_bench }

// Scalar PRNG for comparison
#[bench]
fn small_rng_float(b: &mut Bencher) {
    let mut rng = rand::rngs::SmallRng::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = f64::default();
        for _ in 0..BENCH_N {
            let x: f64 = rng.sample(Open01);
            accum += x;
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<f64>() as u64;
}

#[allow(non_snake_case)]
#[bench]
fn AesRand_256_float(b: &mut Bencher) {
    let mut rng = simd_prngs::AesRand::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = f32x8::default();
        for _ in 0..BENCH_N / 2 {
            let x: f32x8 = rng.sample(Open01);
            accum += x;
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<f32x8>() as u64;
}
#[allow(non_snake_case)]
#[bench]
fn AesRand_128_unroll_float(b: &mut Bencher) {
    let mut rng = simd_prngs::AesRand::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = f32x4::default();
        for _ in 0..BENCH_N / 2 {
            let x: f32x4 = rng.sample(Open01);
            accum += x;
            let x: f32x4 = rng.sample(Open01);
            accum += x;
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<f32x4>() as u64;
}
