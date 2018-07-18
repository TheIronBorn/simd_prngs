//! The ChaCha random number generator.
//!
//! https://cr.yp.to/chacha.html

use std::simd::*;

use rand_core::le;
use rng_impl::*;

// vertical vector rotations can be better implemented with vector
// shuffles when the rotate distance is a multiple of 8
const ROTL_8: [u32; 16] = [1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 13, 14, 15, 12];
const ROTL_16: [u32; 16] = [2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13];
macro_rules! rotate_left {
    ($x:expr, 8) => {{
        let vec8 = u8x16::from_bits($x);
        let r: u8x16 = unsafe { simd_shuffle16(vec8, vec8, ROTL_8) };
        u32x4::from_bits(r)
    }};
    ($x:expr, 16) => {{
        let vec8 = u8x16::from_bits($x);
        let r: u8x16 = unsafe { simd_shuffle16(vec8, vec8, ROTL_16) };
        u32x4::from_bits(r)
    }};
    ($x:expr, $rot:expr) => {{
        $x.rotate_left($rot)
    }};
}

const CONCAT_512: [u32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

/// 4 rounds of ChaCha
///
/// Not cryptographically strong but still has good statistical quality:
/// <http://pracrand.sourceforge.net/Tests_results.txt>
///
/// A single stream (multiple streams in a single vector are viable with `u32x8`
/// addition: [*Vectorization of ChaCha Stream Cipher*](https://eprint.iacr.org/2013/759.pdf)).
///
/// - Memory: 64 bytes
/// - Speed: around half of [`Ars5`](struct.Ars5.html)
pub struct ChaCha4 {
    a: u32x4,
    b: u32x4,
    c: u32x4,
    d: u32x4,
}

impl ChaCha4 {
    #[inline(always)]
    pub fn generate(&mut self) -> u32x16 {
        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;

        macro_rules! round {
            () => {{
                a += b; d ^= a; d = rotate_left!(d, 16);
                c += d; b ^= c; b = rotate_left!(b, 12);
                a += b; d ^= a; d = rotate_left!(d, 8);
                c += d; b ^= c; b = rotate_left!(b, 7);
            }};
        }

        // avoid unnecessary tail-end shuffle
        round!();

        for _round in 0..4 - 1 {
            b = unsafe { simd_shuffle4(b, b, [1, 2, 3, 0]) };
            c = unsafe { simd_shuffle4(c, c, [2, 3, 0, 1]) };
            d = unsafe { simd_shuffle4(d, d, [3, 0, 1, 2]) };

            round!();
        }

        a += self.a;
        b += self.b;
        c += self.c;
        d += self.d;

        // update 64-bit counter
        self.d = u32x4::from_bits(u64x2::from_bits(self.d) + u64x2::new(1, 0));

        let ab: u32x8 = unsafe { simd_shuffle8(a, b, [0, 1, 2, 3, 4, 5, 6, 7]) };
        let cd: u32x8 = unsafe { simd_shuffle8(c, d, [0, 1, 2, 3, 4, 5, 6, 7]) };
        unsafe { simd_shuffle16(ab, cd, CONCAT_512) }
    }
}

impl SeedableRng for ChaCha4 {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_le = [0u32; 8];
        le::read_u32_into(&seed, &mut seed_le);
        Self {
            a: u32x4::new(0x61707865, 0x3320646E, 0x79622D32, 0x6B206574), // constants
            b: u32x4::new(seed_le[0], seed_le[1], seed_le[2], seed_le[3]), // seed
            c: u32x4::new(seed_le[4], seed_le[5], seed_le[6], seed_le[7]), // seed
            d: u32x4::new(0, 0, 0, 0),                                     // counter
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
/// A single stream (multiple streams in a single vector are viable with `u32x8`
/// addition: [*Vectorization of ChaCha Stream Cipher*](https://eprint.iacr.org/2013/759.pdf)).
///
/// - Memory: 64 bytes
/// - Speed: around 1.3 times [`ChaCha4`](struct.ChaCha4.html)
pub struct ChaChaA4 {
    a: u32x4,
    b: u32x4,
    c: u32x4,
    d: u32x4,
}

impl ChaChaA4 {
    #[inline(always)]
    pub fn generate(&mut self) -> u32x16 {
        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;

        macro_rules! round {
            () => {{
                a += b; d ^= a; d = rotate_left!(d, 16);
                c += d; b ^= c; b = rotate_left!(b, 16); // canonical: 12
                a += b; d ^= a; d = rotate_left!(d, 8);
                c += d; b ^= c; b = rotate_left!(b, 8); // canonical: 7
            }};
        }

        // avoid unnecessary tail-end shuffle
        round!();

        for _round in 0..4 - 1 {
            b = unsafe { simd_shuffle4(b, b, [1, 2, 3, 0]) };
            c = unsafe { simd_shuffle4(c, c, [2, 3, 0, 1]) };
            d = unsafe { simd_shuffle4(d, d, [3, 0, 1, 2]) };

            round!();
        }

        a += self.a;
        b += self.b;
        c += self.c;
        d += self.d;

        // update 64-bit counter
        self.d = u32x4::from_bits(u64x2::from_bits(self.d) + u64x2::new(1, 0));

        let ab: u32x8 = unsafe { simd_shuffle8(a, b, [0, 1, 2, 3, 4, 5, 6, 7]) };
        let cd: u32x8 = unsafe { simd_shuffle8(c, d, [0, 1, 2, 3, 4, 5, 6, 7]) };
        unsafe { simd_shuffle16(ab, cd, CONCAT_512) }
    }
}

impl SeedableRng for ChaChaA4 {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_le = [0u32; 8];
        le::read_u32_into(&seed, &mut seed_le);
        Self {
            a: u32x4::new(0x61707865, 0x3320646E, 0x79622D32, 0x6B206574), // constants
            b: u32x4::new(seed_le[0], seed_le[1], seed_le[2], seed_le[3]), // seed
            c: u32x4::new(seed_le[4], seed_le[5], seed_le[6], seed_le[7]), // seed
            d: u32x4::new(0, 0, 0, 0),                                     // counter
        }
    }
}
