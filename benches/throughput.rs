// A benchmark of PRNG throughput

#![feature(test)]

extern crate packed_simd;
extern crate rand;
extern crate simd_prngs;
extern crate test;

use std::mem;
use test::Bencher;

use packed_simd::*;
use rand::prelude::*;
use simd_prngs::*;

const BENCH_N: u64 = 1 << 10;

macro_rules! make_throughput_bench {
    ($gen:ident, $uty:ident, $fty:ident) => {
        #[allow(non_snake_case)]
        mod $gen {
            use super::*;

            #[bench]
            fn throughput(b: &mut Bencher) {
                let mut rng = simd_prngs::$gen::from_rng(thread_rng()).unwrap();

                b.iter(|| {
                    let mut accum = $uty::default();
                    for _ in 0..BENCH_N {
                        // xor tends to have better throughput than addition
                        accum ^= rng.gen::<$uty>();
                    }
                    accum
                });
                b.bytes = BENCH_N * mem::size_of::<$uty>() as u64;
            }
        }
    };
}

for_each_prng! { make_throughput_bench }

// Scalar PRNG for comparison
#[bench]
fn small_rng_throughput(b: &mut Bencher) {
    let mut rng = rand::rngs::SmallRng::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = u64::default();
        for _ in 0..BENCH_N {
            accum ^= rng.gen::<u64>();
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<u64>() as u64;
}

#[allow(non_snake_case)]
#[bench]
fn AesRand_256_throughput(b: &mut Bencher) {
    let mut rng = simd_prngs::AesRand::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = u32x8::default();
        for _ in 0..BENCH_N / 2 {
            accum ^= rng.gen::<u32x8>();
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<u32x8>() as u64;
}

#[allow(non_snake_case)]
#[bench]
fn AesRand_unroll_128_throughput(b: &mut Bencher) {
    let mut rng = simd_prngs::AesRand::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = u32x4::default();
        for _ in 0..BENCH_N / 2 {
            accum ^= rng.gen::<u32x4>();
            accum ^= rng.gen::<u32x4>();
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<u32x4>() as u64;
}
