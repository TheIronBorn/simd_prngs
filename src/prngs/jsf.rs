use std::simd::*;

use rng_impl::*;

macro_rules! make_jsf {
    ($rng_name:ident, $vector:ident, $x:expr, $z:expr) => {
        pub struct $rng_name {
            a: $vector,
            b: $vector,
            c: $vector,
            d: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                let e = self.a - self.b.rotate_left($x);
                self.a = self.b ^ self.c.rotate_left($z);
                self.b = self.c + self.d;
                self.c = self.d + e;
                self.d = e + self.a;
                self.d
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

                let a = seed[0];
                let b = seed[1];
                let c = seed[2];
                let mut d = seed[3];

                let eq = |x: $vector, n| x.eq($vector::splat(n));
                let ne = |x: $vector, n| x.ne($vector::splat(n));

                // PractRand: block the cycles of length 1
                let cmp = eq(d & 0x80093300, 0);
                d += 1 & $vector::from_bits(cmp & ne(a, 0) & ne(b, 0) & ne(c, 0) & ne(d, 0));
                d += 1 & $vector::from_bits(
                    cmp
                        & eq(a, 0x77777777)
                        & eq(b, 0x55555555)
                        & eq(c, 0x11111111)
                        & eq(d, 0x44444444),
                );
                d += 1 & $vector::from_bits(
                    cmp
                        & eq(a, 0x5591F2E3)
                        & eq(b, 0x69EBA6CD)
                        & eq(c, 0x2A171E3D)
                        & eq(d, 0x3FD48890),
                );
                d += 1 & $vector::from_bits(
                    cmp
                        & eq(a, 0x47CB8D56)
                        & eq(b, 0xAE9B35A7)
                        & eq(c, 0x5C78F4A8)
                        & eq(d, 0x522240FF),
                );

                Ok(Self { a, b, c, d })
            }
        }
    };

    (32bit: $rng_name:ident, $vector:ident) => {
        make_jsf!($rng_name, $vector, 27, 17);
    };

    (64bit: $rng_name:ident, $vector:ident) => {
        make_jsf!($rng_name, $vector, 39, 11);
    };
}

make_jsf! { 32bit: Jsf32x2, u32x2 }
make_jsf! { 32bit: Jsf32x4, u32x4 }
make_jsf! { 32bit: Jsf32x8, u32x8 }
make_jsf! { 32bit: Jsf32x16, u32x16 }

make_jsf! { 64bit: Jsf64x2, u64x2 }
make_jsf! { 64bit: Jsf64x4, u64x4 }
make_jsf! { 64bit: Jsf64x8, u64x8 }
