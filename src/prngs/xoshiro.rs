use rand::AsByteSliceMut as RandAsByteSliceMut;
use rng_impl::*;

/// Used from `blocks_from_rng`
struct Xoshiro256 {
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
}

impl Xoshiro256 {
    // TODO: investigate carry-less multiplication implementation
    //       per the paper http://vigna.di.unimi.it/ftp/papers/ScrambledLinear.pdf
    fn jump(&mut self) {
        const JUMP: [u64; 4] = [
            0x180ec6d33cfd0aba,
            0xd5a61266f0c9392c,
            0xa9582618e03fc9aa,
            0x39abdc4529b1661c,
        ];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        for jump in &JUMP {
            for b in 0..64 {
                if (jump & 1 << b) != 0 {
                    s0 ^= self.s0;
                    s1 ^= self.s1;
                    s2 ^= self.s2;
                    s3 ^= self.s3;
                }

                let t = self.s1 << 17;

                self.s2 ^= self.s0;
                self.s3 ^= self.s1;
                self.s1 ^= self.s2;
                self.s0 ^= self.s3;

                self.s2 ^= t;

                self.s3 = self.s3.rotate_left(45);
            }
        }
        self.s0 = s0;
        self.s1 = s1;
        self.s2 = s2;
        self.s3 = s3;
    }
}

macro_rules! make_xoshiro256 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
            s2: $vector,
            s3: $vector,
        }

        impl $rng_name {
            pub fn blocks_from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [0; 4];
                while seed.iter().all(|&x| x == 0) {
                    rng.try_fill_bytes(seed.as_byte_slice_mut())?;
                }

                let mut scalar = Xoshiro256 {
                    s0: seed[0],
                    s1: seed[1],
                    s2: seed[2],
                    s3: seed[3],
                };

                let mut s0 = $vector::splat(scalar.s0);
                let mut s1 = $vector::splat(scalar.s1);
                let mut s2 = $vector::splat(scalar.s2);
                let mut s3 = $vector::splat(scalar.s3);

                for i in 1..$vector::lanes() {
                    // Each stream has 2^128 values before it begins to repeat
                    // the next stream (except the last stream). For more
                    // space in-between streams, use more jumps per stream.
                    // There is also a "long_jump" which jumps by 2^192.
                    scalar.jump();
                    s0 = s0.replace(i, scalar.s0);
                    s1 = s1.replace(i, scalar.s1);
                    s2 = s2.replace(i, scalar.s2);
                    s3 = s3.replace(i, scalar.s3);
                }

                Ok(Self { s0, s1, s2, s3 })
            }
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                // The `++` scrambler might be faster (multiplication,
                // particularly 64-bit, is slow with SIMD. The multiplications could be replaced
                // with a series of shifts and additions but LLVM currently prefers
                // multiplication).
                //
                // The paper suggests the rotate could be replaced by
                // `x ^= x >> rot`. Perhaps even a single byte vector shuffle?
                // (only a one bit difference)
                let result_starstar = (self.s1 * 5).rotate_left_opt(7) * 9;

                let t = self.s1 << 17;

                self.s2 ^= self.s0;
                self.s3 ^= self.s1;
                self.s1 ^= self.s2;
                self.s0 ^= self.s3;

                self.s2 ^= t;

                self.s3 = self.s3.rotate_left_opt(45);

                result_starstar
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seeds = [$vector::default(); 4];
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
                    s2: seeds[2],
                    s3: seeds[3],
                })
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (jumping is possible)
#[rustfmt::skip]
// Listing probability of overlap somewhere:                            Probability
make_xoshiro256! { Xoshiro256StarStarX2, u64x2 } // ≈ 2^2 * l / 2^256 ≈ l * 2^-254
make_xoshiro256! { Xoshiro256StarStarX4, u64x4 } // ≈ 4^2 * l / 2^256 ≈ l * 2^-252
make_xoshiro256! { Xoshiro256StarStarX8, u64x8 } // ≈ 8^2 * l / 2^256 ≈ l * 2^-250

/// Used for `blocks_from_rng`
struct Xoshiro128 {
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
}

impl Xoshiro128 {
    // TODO: investigate carry-less multiplication implementation
    //       per the paper http://vigna.di.unimi.it/ftp/papers/ScrambledLinear.pdf
    fn jump(&mut self) {
        const JUMP: [u32; 4] = [0x8764000b, 0xf542d2d3, 0x6fa035c3, 0x77f2db5b];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        for jump in &JUMP {
            for b in 0..32 {
                if (jump & 1 << b) != 0 {
                    s0 ^= self.s0;
                    s1 ^= self.s1;
                    s2 ^= self.s2;
                    s3 ^= self.s3;
                }

                let t = self.s1 << 9;

                self.s2 ^= self.s0;
                self.s3 ^= self.s1;
                self.s1 ^= self.s2;
                self.s0 ^= self.s3;

                self.s2 ^= t;

                self.s3 = self.s3.rotate_left(11);
            }
        }
        self.s0 = s0;
        self.s1 = s1;
        self.s2 = s2;
        self.s3 = s3;
    }
}

macro_rules! make_xoshiro128 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
            s2: $vector,
            s3: $vector,
        }

        impl $rng_name {
            pub fn blocks_from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [0; 4];
                while seed.iter().all(|&x| x == 0) {
                    rng.try_fill_bytes(seed.as_byte_slice_mut())?;
                }

                let mut scalar = Xoshiro128 {
                    s0: seed[0],
                    s1: seed[1],
                    s2: seed[2],
                    s3: seed[3],
                };

                let mut s0 = $vector::splat(scalar.s0);
                let mut s1 = $vector::splat(scalar.s1);
                let mut s2 = $vector::splat(scalar.s2);
                let mut s3 = $vector::splat(scalar.s3);

                for i in 1..$vector::lanes() {
                    // Each stream has 2^64 values before it begins to repeat
                    // the next stream (except the last stream). For more
                    // space in-between streams, use more jumps per stream.
                    scalar.jump();
                    s0 = s0.replace(i, scalar.s0);
                    s1 = s1.replace(i, scalar.s1);
                    s2 = s2.replace(i, scalar.s2);
                    s3 = s3.replace(i, scalar.s3);
                }

                Ok(Self { s0, s1, s2, s3 })
            }
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                // 32-bit multiplication might be fast enough, but the `++` scrambler might be
                // still be faster.
                let result_starstar = (self.s1 * 5).rotate_left_opt(7) * 9;

                let t = self.s1 << 9;

                self.s2 ^= self.s0;
                self.s3 ^= self.s1;
                self.s1 ^= self.s2;
                self.s0 ^= self.s3;

                self.s2 ^= t;

                self.s3 = self.s3.rotate_left_opt(11);

                result_starstar
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seeds = [$vector::default(); 4];
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
                    s2: seeds[2],
                    s3: seeds[3],
                })
            }
        }
    };
}

make_xoshiro128! { Xoshiro128StarStarX2,  u32x2  } // ≈ 2^2 * l / 2^128 ≈ l * 2^-126
make_xoshiro128! { Xoshiro128StarStarX4,  u32x4  } // ≈ 4^2 * l / 2^128 ≈ l * 2^-124
make_xoshiro128! { Xoshiro128StarStarX8,  u32x8  } // ≈ 8^2 * l / 2^128 ≈ l * 2^-122
make_xoshiro128! { Xoshiro128StarStarX16, u32x16 } // ≈ 8^2 * l / 2^128 ≈ l * 2^-120

macro_rules! make_xoshiro512 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s: [$vector; 8],
        }

        impl_rngcore! { $rng_name }

        impl SimdRng for $rng_name {
            type Result = $vector;

            #[inline(always)]
            fn generate(&mut self) -> $vector {
                let result_starstar = (self.s[1] * 5).rotate_left_opt(7) * 9;

                let t = self.s[1] << 11;

                self.s[2] ^= self.s[0];
                self.s[5] ^= self.s[1];
                self.s[1] ^= self.s[2];
                self.s[7] ^= self.s[3];
                self.s[3] ^= self.s[4];
                self.s[4] ^= self.s[5];
                self.s[0] ^= self.s[6];
                self.s[6] ^= self.s[7];

                self.s[6] ^= t;

                self.s[7] = self.s[7].rotate_left_opt(21);

                result_starstar
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!("`SeedableRng::from_seed` is unimplemented for some PRNG families")
            }

            fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
                let mut seeds = [$vector::default(); 8];
                while seeds
                    .iter()
                    .fold($vector::splat(0), |mask, &s| mask | s)
                    .eq($vector::splat(0))
                    .any()
                {
                    rng.try_fill_bytes(seeds.as_byte_slice_mut())?;
                }

                Ok(Self { s: seeds })
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (jumping is possible)
#[rustfmt::skip]
// Listing probability of overlap somewhere:                            Probability
make_xoshiro512! { Xoshiro512StarStarX2, u64x2 } // ≈ 2^2 * l / 2^512 ≈ l * 2^-510
make_xoshiro512! { Xoshiro512StarStarX4, u64x4 } // ≈ 4^2 * l / 2^512 ≈ l * 2^-508
make_xoshiro512! { Xoshiro512StarStarX8, u64x8 } // ≈ 8^2 * l / 2^512 ≈ l * 2^-506
