#![feature(test)]

extern crate test;
extern crate rand;
extern crate simd_prngs;

use test::Bencher;
use rand::prelude::*;

use simd_prngs::*;

// benchmark PRNG initialization
macro_rules! init {
    ($fnn:ident, $gen:ident, $init:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            #[allow(deprecated)]
            let mut rng = rand::XorShiftRng::from_rng(thread_rng()).unwrap();
            b.iter(|| {
                let r2 = $gen::$init(&mut rng).unwrap();
                r2
            });
        }
    };
}

init! { init_jumps_xoroshiro128starstar_x2, Xoroshiro128StarStarX2, blocks_from_rng }
init! { init_jumps_xoroshiro128starstar_x4, Xoroshiro128StarStarX4, blocks_from_rng }
init! { init_jumps_xoroshiro128starstar_x8, Xoroshiro128StarStarX8, blocks_from_rng }

init! { init_rand_xoroshiro128starstar_x4, Xoroshiro128StarStarX4, from_rng }
init! { init_rand_xoroshiro128starstar_x2, Xoroshiro128StarStarX2, from_rng }
init! { init_rand_xoroshiro128starstar_x8, Xoroshiro128StarStarX8, from_rng }

init! { init_jumps_xoshiro256starstar_x2, Xoshiro256StarStarX2, blocks_from_rng }
init! { init_jumps_xoshiro256starstar_x4, Xoshiro256StarStarX4, blocks_from_rng }
init! { init_jumps_xoshiro256starstar_x8, Xoshiro256StarStarX8, blocks_from_rng }

init! { init_rand_xoshiro256starstar_x2, Xoshiro256StarStarX2, from_rng }
init! { init_rand_xoshiro256starstar_x4, Xoshiro256StarStarX4, from_rng }
init! { init_rand_xoshiro256starstar_x8, Xoshiro256StarStarX8, from_rng }

/*init! { init_block_xsm32_x2, Xsm32x2, blocks_from_rng }
init! { init_rand_xsm32_x2, Xsm32x2, from_rng }
init! { init_block_xsm32_x4, Xsm32x4, blocks_from_rng }
init! { init_rand_xsm32_x4, Xsm32x4, from_rng }
init! { init_block_xsm32_x8, Xsm32x8, blocks_from_rng }
init! { init_rand_xsm32_x8, Xsm32x8, from_rng }
init! { init_block_xsm32_x16, Xsm32x16, blocks_from_rng }
init! { init_rand_xsm32_x16, Xsm32x16, from_rng }*/
