use rand::AsByteSliceMut as RandAsByteSliceMut;
use rng_impl::*;

macro_rules! make_xoroshiro {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
        }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                let s0 = self.s0;
                let mut s1 = self.s1;
                // The `++` scrambler might be faster (multiplication,
                // particularly 64-bit, is slow with SIMD. The multiplications could be replaced
                // with a series of shifts and additions but LLVM currently prefers
                // multiplication).
                //
                // The paper suggests the rotate could be replaced by
                // `x ^= x >> rot`. Perhaps even a single byte vector shuffle?
                // (only a one bit difference)
                let result = (s0 * 5).rotate_left_opt(7) * 9;

                s1 ^= s0;
                // this rotate could be implemented as a shuffle (as it is divisible by 8)
                self.s0 = s0.rotate_left_opt(24) ^ s1 ^ (s1 << 16); // a, b
                self.s1 = s1.rotate_left_opt(37); // c

                result
            }
        }

        impl_rngcore! { $rng_name }

        impl $rng_name {
            pub fn blocks_from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                struct Xoroshiro128 {
                    s0: u64,
                    s1: u64,
                }

                impl Xoroshiro128 {
                    // TODO: investigate carry-less multiplication implementation
                    //       per the paper http://vigna.di.unimi.it/ftp/papers/ScrambledLinear.pdf
                    fn jump(&mut self) {
                        const JUMP: [u64; 2] = [0xdf900294d8f554a5, 0x170865df4b3201fc];

                        let mut s0 = 0;
                        let mut s1 = 0;
                        for jump in &JUMP {
                            for b in 0..64 {
                                if (jump & 1 << b) != 0 {
                                    s0 ^= self.s0;
                                    s1 ^= self.s1;
                                }

                                let s0 = self.s0;
                                let mut s1 = self.s1;

                                s1 ^= s0;
                                self.s0 = s0.rotate_left(24) ^ s1 ^ (s1 << 16); // a, b
                                self.s1 = s1.rotate_left(37); // c
                            }
                        }
                        self.s0 = s0;
                        self.s1 = s1;
                    }
                }

                let mut seed = [0_u64; 2];
                while seed.iter().all(|&x| x == 0) {
                    rng.try_fill_bytes(seed.as_byte_slice_mut())?;
                }

                let mut scalar = Xoroshiro128 {
                    s0: seed[0],
                    s1: seed[1],
                };

                let mut s0 = $vector::splat(scalar.s0);
                let mut s1 = $vector::splat(scalar.s1);

                for i in 1..$vector::lanes() {
                    // Each stream has 2^64 values before it begins to repeat
                    // the next stream (except the last stream). For more
                    // space in-between streams, use more jumps per stream.
                    // There is also a "long_jump" which jumps by 2^96.
                    scalar.jump();
                    s0 = s0.replace(i, scalar.s0);
                    s1 = s1.replace(i, scalar.s1);
                }

                Ok(Self { s0, s1 })
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seeds = [$vector::default(); 2];
                while seeds
                    .iter()
                    .fold($vector::splat(0), |mask, &s| mask | s)
                    .eq($vector::splat(0))
                    .any()
                {
                    rng.try_fill_bytes(seeds.as_byte_slice_mut())?;
                }

                Ok(Self {
                    s0: seeds[0],
                    s1: seeds[1],
                })
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (jumping is possible)
#[rustfmt::skip]
// Listing probability of overlap somewhere:                             Probability
make_xoroshiro! { Xoroshiro128StarStarX2, u64x2 } // ≈ 2^2 * l / 2^128 ≈ l * 2^-126
make_xoroshiro! { Xoroshiro128StarStarX4, u64x4 } // ≈ 4^2 * l / 2^128 ≈ l * 2^-124
make_xoroshiro! { Xoroshiro128StarStarX8, u64x8 } // ≈ 8^2 * l / 2^128 ≈ l * 2^-122
