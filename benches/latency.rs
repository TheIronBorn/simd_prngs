//! A benchmark of PRNG latency

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

// This benchmark emulates rejection sampling, so the number of RNG calls varies. For meaningful
// data, it must be run many times.
const BENCH_N: u64 = 1 << 15;

const BOUND_8: u64 = 0;
const BOUND_16: u64 = 0x00FF;
const BOUND_32: u64 = 0x00FF_FFFF;
const BOUND_64: u64 = 0x00FF_FFFF_FFFF_FFFF;

macro_rules! make_latency_bench {
    ($gen:ident, $uty:ident, $fty:ident) => {
        #[allow(non_snake_case)]
        mod $gen {
            use super::*;

            #[bench]
            fn latency(b: &mut Bencher) {
                const LANE_WIDTH: usize = mem::size_of::<$uty>() * 8 / $uty::lanes();

                let mut rng = simd_prngs::$gen::from_rng(thread_rng()).unwrap();
                let bound = match LANE_WIDTH {
                    8 => BOUND_8,
                    16 => BOUND_16,
                    32 => BOUND_32,
                    64 => BOUND_64,
                    _ => unreachable!(),
                };

                b.iter(|| {
                    let mut accum = $uty::splat(0);
                    for _ in 0..BENCH_N {
                        loop {
                            let rand = rng.gen::<$uty>();
                            // for any lane-width, 1/256 chance to loop
                            if rand.extract(0) as u64 > bound {
                                accum ^= rand;
                                break;
                            }
                        }
                    }
                    accum
                });
                b.bytes = BENCH_N * mem::size_of::<$uty>() as u64;
            }
        }
    };
}

for_each_prng! { make_latency_bench }

// Scalar PRNG for comparison
#[bench]
fn small_rng_reject(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = 0;
        for _ in 0..BENCH_N {
            loop {
                let rand = rng.gen::<u64>();
                if rand as u64 > BOUND_64 {
                    accum ^= rand;
                    break;
                }
            }
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<i64>() as u64;
}

#[bench]
#[allow(non_snake_case)]
fn AesRand_256_latency(b: &mut Bencher) {
    let mut rng = simd_prngs::AesRand::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = u32x8::splat(0);
        for _ in 0..BENCH_N {
            loop {
                let rand = rng.gen::<u32x8>();
                if rand.extract(0) as u64 > BOUND_32 {
                    accum ^= rand;
                    break;
                }
            }
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<u32x8>() as u64;
}

#[bench]
#[allow(non_snake_case)]
fn AesRand_unroll_128_latency(b: &mut Bencher) {
    let mut rng = simd_prngs::AesRand::from_rng(thread_rng()).unwrap();

    b.iter(|| {
        let mut accum = u32x4::splat(0);
        for _ in 0..BENCH_N {
            loop {
                let rand = rng.gen::<u32x4>();
                if rand.extract(0) as u64 > BOUND_32 {
                    accum ^= rand;
                    break;
                }
                let rand = rng.gen::<u32x4>();
                if rand.extract(0) as u64 > BOUND_32 {
                    accum ^= rand;
                    break;
                }
            }
        }
        accum
    });
    b.bytes = BENCH_N * mem::size_of::<u32x4>() as u64;
}
