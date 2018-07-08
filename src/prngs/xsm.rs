use std::simd::*;

use rng_impl::*;

macro_rules! make_xsm32 {
    ($rng_name:ident, $vec:ident) => {
        pub struct $rng_name {
            lcg_low: $vec,
            lcg_high: $vec,
            lcg_adder: $vec,
            history: $vec,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vec {
                const K: u32 = 0x6595a395;

                let mut rv = self.history * 0x6595a395;
                let mut tmp = self.lcg_high + (self.lcg_high ^ self.lcg_low).rotate_left(11);
                tmp *= K;

                let mut old_lcg_low = self.lcg_low;
                self.lcg_low += self.lcg_adder;
                old_lcg_low -= $vec::from_bits(self.lcg_low.lt(self.lcg_adder)); // += (x < y) as usize
                self.lcg_high += old_lcg_low;

                rv ^= rv >> 16;
                tmp ^= tmp >> 16;
                self.history = tmp;
                rv += self.history;
                rv
            }

            /// There's little documentation so I'm unclear how best to
            /// implement this.
            pub fn blocks_from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                struct Xsm32 {
                    lcg_low: u32,
                    lcg_high: u32,
                    lcg_adder: u32,
                }

                impl Xsm32 {
                    fn seek_forward(&mut self) {
                        fn fast_forward_lcg64(mut val: u64, mut mul: u64, mut add: u64) -> u64 {
                            let mut how_far: u32 = 0xffff_ffff; // 2^32 - 1

                            loop {
                                if how_far & 1 != 0 {
                                    val = val.wrapping_mul(mul).wrapping_add(add);
                                }
                                how_far >>= 1;
                                if how_far == 0 { break; }
                                add = add.wrapping_mul(mul).wrapping_add(add);
                                mul = mul.wrapping_mul(mul);
                            }
                            val
                        }

                        // how_far -= 1; 2^x - 1
                        let mut lcg_state = self.lcg_low as u64 | ((self.lcg_high as u64) << 32);
                        let mul = 0x0000_0001_0000_0001;
                        lcg_state = fast_forward_lcg64(lcg_state, mul, self.lcg_adder as u64);
                        self.lcg_low = lcg_state as u32;
                        self.lcg_high = (lcg_state >> 32) as u32;

                        // xsm.generate();
                        let mut old_lcg_low = self.lcg_low;
                        self.lcg_low += self.lcg_adder;
                        old_lcg_low += (self.lcg_low < self.lcg_adder) as u32;
                        self.lcg_high += old_lcg_low;
                    }
                }

                let mut seeds = [0; 3];
                rng.try_fill(&mut seeds)?;

                let mut scalar = Xsm32 {
                    lcg_high: seeds[0] | 1,
                    lcg_adder: seeds[1],
                    lcg_low: seeds[2],
                };

                let mut lcg_low = $vec::splat(scalar.lcg_low);
                let mut lcg_high = $vec::splat(scalar.lcg_high);
                let mut lcg_adder = $vec::splat(scalar.lcg_adder);

                for i in 1..$vec::lanes() {
                    // Each stream has 2^32 values before it begins to repeat
                    // the next stream (except the last stream). For more
                    // space in-between streams, use more jumps per stream.
                    scalar.seek_forward();
                    lcg_low = lcg_low.replace(i, scalar.lcg_low);
                    lcg_high = lcg_high.replace(i, scalar.lcg_high);
                    lcg_adder = lcg_adder.replace(i, scalar.lcg_adder);
                }

                Ok(Self { lcg_low, lcg_high, lcg_adder, history: $vec::splat(0) })
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                let mut seeds = [$vec::default(); 3];
                rng.try_fill(seeds.as_byte_slice_mut())?;

                let mut xsm = Self {
                    lcg_high: seeds[0] | 1,
                    lcg_adder: seeds[1],
                    lcg_low: seeds[2],
                    history: $vec::splat(0),
                };

                xsm.generate();

                Ok(xsm)
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters *might* be possible)
// (jumping is possible)
// Listing probability of overlap somewhere:          Probability
make_xsm32! { Xsm32x2, u32x2 } // 2^2 * l / 2^64 ≈    l * 2^-62
make_xsm32! { Xsm32x4, u32x4 } // 4^2 * l / 2^64 ≈    l * 2^-60
make_xsm32! { Xsm32x8, u32x8 } // 8^2 * l / 2^64 ≈    l * 2^-58
make_xsm32! { Xsm32x16, u32x16 } // 16^2 * l / 2^64 ≈ l * 2^-56

macro_rules! make_xsm64 {
    ($rng_name:ident, $vec:ident, $vec64:ident) => {
        pub struct $rng_name {
            lcg_low: $vec,
            lcg_high: $vec,
            lcg_adder: $vec,
            history: $vec,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vec {
                const K: u64 = 0xA3EC647659359ACD;

                self.history *= K;
                let mut tmp = self.lcg_high + (self.lcg_high ^ self.lcg_low).rotate_left(19);
                tmp *= K;

                let mut old = self.lcg_low;
                self.lcg_low += self.lcg_adder;
                self.lcg_high += old - $vec::from_bits(self.lcg_low.lt(self.lcg_adder));  // += (x < y) as usize

                old = self.history ^ (self.history >> 32);
                tmp ^= tmp >> 32;
                self.history = tmp;
                tmp + old
            }

            /*pub fn blocks_from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                fn fast_forward_lcg128(
                    how_far_low: u64,
                    how_far_high: u64,
                    value_low: &mut u64,
                    value_high: &mut u64,
                    mul_low: u64,
                    mul_high: u64,
                    add_low: u64,
                    add_high: u64,
                ) {
                    Uint32 value[4], mul[4], add[4], tmp[4];
                    convert128_64to32(value_low, value_high, value);
                    convert128_64to32(mul_low, mul_high, mul);
                    convert128_64to32(add_low, add_high, add);
                    while (1) {
                        if (how_far_low & 1) {
                            multiply_128(value, mul, value);
                            add_128(value, add, value);
                            //val = val * mul + add;
                        }
                        how_far_low = (how_far_low >> 1) | (how_far_high << 63);
                        how_far_high >>= 1;
                        if (how_far_low == 0 && how_far_high == 0) break;
                        multiply_128(add, mul, tmp);
                        add_128(add, tmp, add);
                        //add = add * mul + add;
                        multiply_128(mul, mul, mul);
                        //mul = mul * mul;
                    }
                    value_low  = value[0] | (Uint64(value[1]) << 32);
                    value_high = value[2] | (Uint64(value[3]) << 32);
                    return;
                }

                let mut seeds = [0u32; 3];
                rng.try_fill(&mut seeds)?;

                let mut xsm = Self {
                    lcg_high: seeds[0] | 1,
                    lcg_adder: seeds[1],
                    lcg_low: seeds[2],
                    history: 0,
                };

                // if (!how_far_low && !how_far_high) return;
                // if (!how_far_low--) how_far_high--;
                fast_forward_lcg128(/*how_far_low, how_far_high,*/
                    xsm.lcg_low, xsm.lcg_high, 1, 1, xsm.lcg_adder, 0
                );
                xsm.generate();

                Ok(xsm)
            }*/
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                let mut seeds = [$vec::default(); 2];
                rng.try_fill(seeds.as_byte_slice_mut())?;

                let mut xsm = Self {
                    lcg_high: seeds[0] | 1,
                    lcg_adder: seeds[1],
                    lcg_low: $vec::splat(0),
                    history: $vec::splat(0),
                };

                xsm.generate();

                Ok(xsm)
            }
        }
    };
}

// (where `l` is stream length)
// (multiple parameters *might* be possible)
// (jumping is possible)
// Listing probability of overlap somewhere:               Probability
make_xsm64! { Xsm64x2, u64x2, u64x2 } // 2^2 * l / 2^128 ≈ l * 2^-126
make_xsm64! { Xsm64x4, u64x4, u64x4 } // 4^2 * l / 2^128 ≈ l * 2^-124
make_xsm64! { Xsm64x8, u64x8, u64x8 } // 8^2 * l / 2^128 ≈ l * 2^-122
