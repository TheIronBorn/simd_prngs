use std::simd::*;

use rng_impl::*;

macro_rules! make_lfsr113 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            z1: $vector,
            z2: $vector,
            z3: $vector,
            z4: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                let mut b;
                b = ((self.z1 << 6) ^ self.z1) >> 13;
                self.z1 = ((self.z1 & 4294967294) << 18) ^ b;
                b = ((self.z2 << 2) ^ self.z2) >> 27;
                self.z2 = ((self.z2 & 4294967288) << 2) ^ b;
                b = ((self.z3 << 13) ^ self.z3) >> 21;
                self.z3 = ((self.z3 & 4294967280) << 7) ^ b;
                b = ((self.z4 << 3) ^ self.z4) >> 12;
                self.z4 = ((self.z4 & 4294967168) << 13) ^ b;
                self.z1 ^ self.z2 ^ self.z3 ^ self.z4
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 4];
                rng.try_fill(seed.as_byte_slice_mut())?;

                // could perhaps use the seeding of ISPC
                while seed[0].le($vector::splat(1)).any() {
                    rng.try_fill(seed[0..0 + 1].as_byte_slice_mut())?;
                }
                while seed[1].le($vector::splat(7)).any() {
                    rng.try_fill(seed[1..1 + 1].as_byte_slice_mut())?;
                }
                while seed[2].le($vector::splat(15)).any() {
                    rng.try_fill(seed[2..2 + 1].as_byte_slice_mut())?;
                }
                while seed[3].le($vector::splat(127)).any() {
                    rng.try_fill(seed[3..3 + 1].as_byte_slice_mut())?;
                }

                Ok(Self {
                    z1: seed[0],
                    z2: seed[1],
                    z3: seed[2],
                    z4: seed[3],
                })
            }
        }
    };
}

make_lfsr113! { Lfsr113x2, u32x2 }
make_lfsr113! { Lfsr113x4, u32x4 }
make_lfsr113! { Lfsr113x8, u32x8 }
make_lfsr113! { Lfsr113x16, u32x16 }

macro_rules! make_lfsr258 {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            y1: $vector,
            y2: $vector,
            y3: $vector,
            y4: $vector,
            y5: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                let mut b;

                b = ((self.y1 << 1) ^ self.y1) >> 53;
                self.y1 = ((self.y1 & 18446744073709551614) << 10) ^ b;
                b = ((self.y2 << 24) ^ self.y2) >> 50;
                self.y2 = ((self.y2 & 18446744073709551104) << 5) ^ b;
                b = ((self.y3 << 3) ^ self.y3) >> 23;
                self.y3 = ((self.y3 & 18446744073709547520) << 29) ^ b;
                b = ((self.y4 << 5) ^ self.y4) >> 24;
                self.y4 = ((self.y4 & 18446744073709420544) << 23) ^ b;
                b = ((self.y5 << 3) ^ self.y5) >> 33;
                self.y5 = ((self.y5 & 18446744073701163008) << 8) ^ b;
                self.y1 ^ self.y2 ^ self.y3 ^ self.y4 ^ self.y5
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                let mut seed = [$vector::default(); 5];
                rng.try_fill(seed.as_byte_slice_mut())?;

                while seed[0].le($vector::splat(1)).any() {
                    rng.try_fill(seed[0..0 + 1].as_byte_slice_mut())?;
                }
                while seed[1].le($vector::splat(511)).any() {
                    rng.try_fill(seed[1..1 + 1].as_byte_slice_mut())?;
                }
                while seed[2].le($vector::splat(4095)).any() {
                    rng.try_fill(seed[2..2 + 1].as_byte_slice_mut())?;
                }
                while seed[3].le($vector::splat(131071)).any() {
                    rng.try_fill(seed[3..3 + 1].as_byte_slice_mut())?;
                }
                while seed[4].le($vector::splat(8388607)).any() {
                    rng.try_fill(seed[4..4 + 1].as_byte_slice_mut())?;
                }

                Ok(Self {
                    y1: seed[0],
                    y2: seed[1],
                    y3: seed[2],
                    y4: seed[3],
                    y5: seed[4],
                })
            }
        }
    };
}

make_lfsr258! { Lfsr258x2, u64x2 }
make_lfsr258! { Lfsr258x4, u64x4 }
make_lfsr258! { Lfsr258x8, u64x8 }
