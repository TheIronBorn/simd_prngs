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
make_xsm32! { Xsm32x2, u32x2 } // 2^2 * l / 2^64 =    l * 2^-62
make_xsm32! { Xsm32x4, u32x4 } // 4^2 * l / 2^64 =    l * 2^-60
make_xsm32! { Xsm32x8, u32x8 } // 8^2 * l / 2^64 =    l * 2^-58
make_xsm32! { Xsm32x16, u32x16 } // 16^2 * l / 2^64 = l * 2^-56

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
make_xsm64! { Xsm64x2, u64x2, u64x2 } // 2^2 * l / 2^128 = l * 2^-126
make_xsm64! { Xsm64x4, u64x4, u64x4 } // 4^2 * l / 2^128 = l * 2^-124
make_xsm64! { Xsm64x8, u64x8, u64x8 } // 8^2 * l / 2^128 = l * 2^-122
