//! The ChaCha random number generator.
//!
//! https://cr.yp.to/chacha.html

use rng_impl::*;

const CHACHA_SEED: u32x4 = u32x4::new(0x61707865, 0x3320646E, 0x79622D32, 0x6B206574);

/// 4 rounds of ChaCha
///
/// Not cryptographically strong but still has good statistical quality:
/// <http://pracrand.sourceforge.net/Tests_results.txt>
///
/// A single stream.
///
/// Multiple streams in a single vector is viable with AVX2:
/// [*Vectorization of ChaCha Stream Cipher*](https://eprint.iacr.org/2013/759.pdf)).
///
/// - Memory: 64 bytes
/// - Speed: around half of [`Ars5`](struct.Ars5.html)
pub struct ChaCha4 {
    a: u32x4,
    b: u32x4,
    c: u32x4,
    d: u32x4,
}

impl_rngcore! { ChaCha4 }

impl SimdRng for ChaCha4 {
    type Result = u32x16;

    #[inline(always)]
    fn generate(&mut self) -> u32x16 {
        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;

        #[rustfmt::skip]
        macro_rules! round {
            () => {{
                a += b; d ^= a; d = d.rotate_left_opt(16);
                c += d; b ^= c; b = b.rotate_left_opt(12);
                a += b; d ^= a; d = d.rotate_left_opt(8);
                c += d; b ^= c; b = b.rotate_left_opt(7);
            }};
        }

        // avoid unnecessary tail-end shuffle
        round!();

        for _round in 0..4 - 1 {
            b = shuffle!(b, b, [1, 2, 3, 0]);
            c = shuffle!(c, c, [2, 3, 0, 1]);
            d = shuffle!(d, d, [3, 0, 1, 2]);

            round!();
        }

        a += self.a;
        b += self.b;
        c += self.c;
        d += self.d;

        // update 64-bit counter
        self.d = u32x4::from_bits(u64x2::from_bits(self.d) + 1);

        let ab: u32x8 = shuffle!(a, b, [0, 1, 2, 3, 4, 5, 6, 7]);
        let cd: u32x8 = shuffle!(c, d, [0, 1, 2, 3, 4, 5, 6, 7]);
        shuffle!(
            ab,
            cd,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        )
    }
}

impl SeedableRng for ChaCha4 {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            a: CHACHA_SEED,                                                // constants
            b: u32x4::from_bits(u8x16::from_slice_unaligned(&seed[..16])), // seed
            c: u32x4::from_bits(u8x16::from_slice_unaligned(&seed[16..])), // seed
            d: u32x4::splat(0),                                            // counter
        }
    }
}

/// A faster variant of [`ChaCha4`](struct.ChaCha4.html)
///
/// Rotate distances are changed from the canonical ChaCha algorithm for
/// increased speed with vector shuffles (1 instruction rather than 3). This
/// variant fails PractRand at 1 TB.
///
/// "The exact choice of distances doesn’t seem very important"
/// \- <https://cr.yp.to/snuffle/salsafamily-20071225.pdf#page=12> (section 4.6)
///
/// Not cryptographically strong but still has good statistical quality:
/// <http://pracrand.sourceforge.net/Tests_results.txt>
///
/// A single stream.
///
/// Multiple streams in a single vector is viable with AVX2:
/// [*Vectorization of ChaCha Stream Cipher*](https://eprint.iacr.org/2013/759.pdf)).
///
/// - Memory: 64 bytes
/// - Speed: around 1.3 times [`ChaCha4`](struct.ChaCha4.html)
pub struct ChaChaAlt4 {
    a: u32x4,
    b: u32x4,
    c: u32x4,
    d: u32x4,
}

impl_rngcore! { ChaChaAlt4 }

impl SimdRng for ChaChaAlt4 {
    type Result = u32x16;

    #[inline(always)]
    fn generate(&mut self) -> u32x16 {
        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;

        #[rustfmt::skip]
        macro_rules! round {
            () => {{
                a += b; d ^= a; d = d.rotate_left_opt(16);
                c += d; b ^= c; b = b.rotate_left_opt(16); // canonical: 12
                a += b; d ^= a; d = d.rotate_left_opt(8);
                c += d; b ^= c; b = b.rotate_left_opt(8); // canonical: 7
            }};
        }

        // avoid unnecessary tail-end shuffle
        round!();

        for _round in 0..4 - 1 {
            b = shuffle!(b, b, [1, 2, 3, 0]);
            c = shuffle!(c, c, [2, 3, 0, 1]);
            d = shuffle!(d, d, [3, 0, 1, 2]);

            round!();
        }

        a += self.a;
        b += self.b;
        c += self.c;
        d += self.d;

        // update 64-bit counter
        self.d = u32x4::from_bits(u64x2::from_bits(self.d) + 1);

        // panic!();
        let ab: u32x8 = shuffle!(a, b, [0, 1, 2, 3, 4, 5, 6, 7]);
        let cd: u32x8 = shuffle!(c, d, [0, 1, 2, 3, 4, 5, 6, 7]);
        shuffle!(
            ab,
            cd,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        )
    }
}

impl SeedableRng for ChaChaAlt4 {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            a: CHACHA_SEED,                                                // constants
            b: u32x4::from_bits(u8x16::from_slice_unaligned(&seed[..16])), // seed
            c: u32x4::from_bits(u8x16::from_slice_unaligned(&seed[16..])), // seed
            d: u32x4::splat(0),                                            // counter
        }
    }
}
