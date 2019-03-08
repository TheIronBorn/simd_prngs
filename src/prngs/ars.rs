// When only one 128-bit stream is used, there is no worry of correlation. If
// multiple streams are used, it is trivial to avoid correlation by setting
// the `input` counter appropriately
//
// I do not know if a `aarch64` implementation is possible

use std::arch::x86_64::*;

use rng_impl::*;

#[inline(always)]
fn aes_enc(x: u64x2, k: u64x2) -> u64x2 {
    let x = __m128i::from_bits(x);
    let k = __m128i::from_bits(k);

    let r = unsafe { _mm_aesenc_si128(x, k) };
    u64x2::from_bits(r)
}

#[inline(always)]
fn aes_enc_last(x: u64x2, k: u64x2) -> u64x2 {
    let x = __m128i::from_bits(x);
    let k = __m128i::from_bits(k);

    let r = unsafe { _mm_aesenclast_si128(x, k) };
    u64x2::from_bits(r)
}

const KEY_WEYL: u64x2 = u64x2::new(
    0xbb67ae8584caa73b, // sqrt(3) - 1.0
    0x9e3779b97f4a7c15, // golden ratio
);

/// ARS-5 from [Random123]
///
/// A single stream
///
/// 4 rounds is not "Crush-resistant" (ARS-4 gets >256GB with PractRand)
///
/// [Random123]: http://www.deshawresearch.com/resources_random123.html
pub struct Ars5 {
    input: u64x2,
    key: u64x2,
}

impl SimdRng for Ars5 {
    type Result = u64x2;

    #[inline(always)]
    fn generate(&mut self) -> u64x2 {
        let mut kk = self.key;
        let mut v = self.input ^ kk;

        // Do we need exact u128 increment math here? Or is any
        // 2^128-period Weyl-style sequence sufficient? (Currently just
        // SIMD addition for simplicity, unsure of the period).
        self.input += u64x2::new(1, 2);

        // final round is `aes_enc_last`
        for _round in 0..5 - 1 {
            kk += KEY_WEYL;
            v = aes_enc(v, kk);
        }

        kk += KEY_WEYL;
        aes_enc_last(v, kk)
    }
}

impl_rngcore! { Ars5 }

impl SeedableRng for Ars5 {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let load = |x| u64x2::from_bits(u8x16::from_slice_unaligned(x));

        Self {
            input: load(&seed[..16]),
            key: load(&seed[16..]),
        }
    }
}

/// ARS-7 from [Random123]
///
/// A single stream
///
/// [Random123]: http://www.deshawresearch.com/resources_random123.html
pub struct Ars7 {
    input: u64x2,
    key: u64x2,
}

impl SimdRng for Ars7 {
    type Result = u64x2;

    #[inline(always)]
    fn generate(&mut self) -> u64x2 {
        let mut kk = self.key;
        let mut v = self.input ^ kk;

        // Do we need exact u128 increment math here? Or is any
        // 2^128-period Weyl-style sequence sufficient? (Currently just
        // SIMD addition for simplicity, unsure of the period).
        self.input += u64x2::new(1, 2);

        // final round is `aes_enc_last`
        for _round in 0..7 - 1 {
            kk += KEY_WEYL;
            v = aes_enc(v, kk);
        }

        kk += KEY_WEYL;
        aes_enc_last(v, kk)
    }
}

impl_rngcore! { Ars7 }

impl SeedableRng for Ars7 {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let load = |x| u64x2::from_bits(u8x16::from_slice_unaligned(x));

        Self {
            input: load(&seed[..16]),
            key: load(&seed[16..]),
        }
    }
}
