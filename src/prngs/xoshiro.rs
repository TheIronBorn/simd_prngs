use rng_impl::*;

macro_rules! make_xoshiro {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
            s2: $vector,
            s3: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // The `++` scrambler might be faster (multiplication,
                // particularly 64-bit, is slow with SIMD).
                //
                // The paper suggests the rotate could be replaced by
                // `x ^= x >> rot`. Perhaps even a single byte vector shuffle?
                // (only a one bit difference)
                let result_starstar = rotate_left!(self.s1 * 5, 7, $vector) * 9;

                let t = self.s1 << 17;

                self.s2 ^= self.s0;
                self.s3 ^= self.s1;
                self.s1 ^= self.s2;
                self.s0 ^= self.s3;

                self.s2 ^= t;

                self.s3 = rotate_left!(self.s3, 45, $vector);

                result_starstar
            }

            pub fn blocks_from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                struct Xoroshiro128 {
                    s0: u64,
                    s1: u64,
                    s2: u64,
                    s3: u64,
                }

                impl Xoroshiro128 {
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

                let mut seed = [0; 2];
                while seed.iter().all(|&x| x == 0) {
                    rng.try_fill(&mut seed)?;
                }

                let mut scalar = Xoroshiro128 {
                    s0: seed[0],
                    s1: seed[1],
                    s2: seed[1],
                    s3: seed[1],
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

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                const ZERO: $vector = $vector::splat(0);

                let mut seeds = [$vector::default(); 4];
                while seeds
                    .iter()
                    // `splat(true)`
                    .fold(ZERO.eq(ZERO), |acc, s| acc & s.eq(&ZERO))
                    .any()
                {
                    rng.try_fill(seeds.as_byte_slice_mut())?;
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
// Listing probability of overlap somewhere:                       Probability
make_xoshiro! { Xoshiro256StarStarX2, u64x2 } // 2^2 * l / 2^256 ≈ l * 2^-254
make_xoshiro! { Xoshiro256StarStarX4, u64x4 } // 4^2 * l / 2^256 ≈ l * 2^-252
make_xoshiro! { Xoshiro256StarStarX8, u64x8 } // 8^2 * l / 2^256 ≈ l * 2^-250

macro_rules! make_xoshiro512 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
            s2: $vector,
            s3: $vector,
            s4: $vector,
            s5: $vector,
            s6: $vector,
            s7: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                // The `++` scrambler might be faster (multiplication,
                // particularly 64-bit, is slow with SIMD).
                //
                // The paper suggests the rotate could be replaced by
                // `x ^= x >> rot`. Perhaps even a single byte vector shuffle?
                // (only a one bit difference)
                let result_starstar = rotate_left!(self.s1 * 5, 7, $vector) * 9;

                let t = self.s1 << 11;

                self.s2 ^= self.s0;
                self.s5 ^= self.s1;
                self.s1 ^= self.s2;
                self.s7 ^= self.s3;
                self.s3 ^= self.s4;
                self.s4 ^= self.s5;
                self.s0 ^= self.s6;
                self.s6 ^= self.s7;

                self.s6 ^= t;

                self.s7 = rotate_left!(self.s7, 21, $vector);

                result_starstar
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                const ZERO: $vector = $vector::splat(0);

                let mut seeds = [$vector::default(); 8];
                while seeds
                    .iter()
                    // `splat(true)`
                    .fold(ZERO.eq(ZERO), |acc, s| acc & s.eq(&ZERO))
                    .any()
                {
                    rng.try_fill(seeds.as_byte_slice_mut())?;
                }

                Ok(Self {
                    s0: seeds[0],
                    s1: seeds[1],
                    s2: seeds[2],
                    s3: seeds[3],
                    s4: seeds[4],
                    s5: seeds[5],
                    s6: seeds[6],
                    s7: seeds[7],
                })
            }
        }
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
// (where `l` is stream length)
// (multiple parameters could be used, though slow on older hardware)
// (jumping is possible)
// Listing probability of overlap somewhere:                          Probability
make_xoshiro512! { Xoshiro512StarStarX2, u64x2 } // 2^2 * l / 2^512 ≈ l * 2^-510
make_xoshiro512! { Xoshiro512StarStarX4, u64x4 } // 4^2 * l / 2^512 ≈ l * 2^-508
make_xoshiro512! { Xoshiro512StarStarX8, u64x8 } // 8^2 * l / 2^512 ≈ l * 2^-506
