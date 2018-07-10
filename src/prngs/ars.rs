// When only one 128-bit stream is used, there is no worry of correlation. If
// multiple streams are used, it is trivial to avoid correlation by setting
// the `input` counter appropriately
//
// I do not know if a `aarch64` implementation is possible

use std::arch::x86_64::*;
use std::simd::*;

use rng_impl::*;

#[inline]
fn aes_enc(x: u64x2, k: u64x2) -> u64x2 {
    let x = __m128i::from_bits(x);
    let k = __m128i::from_bits(k);

    let r = unsafe { _mm_aesenc_si128(x, k) };
    u64x2::from_bits(r)
}

#[inline]
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
/// 4 rounds is not "Crush-resistant"
///
/// [Random123]: http://www.deshawresearch.com/resources_random123.html
pub struct Ars5 {
    input: u64x2,
    key: u64x2,
}

impl Ars5 {
    #[inline(always)]
    pub fn generate(&mut self) -> u64x2 {
        let mut kk = self.key;
        let mut v = self.input ^ kk;

        // Do we need exact u128 increment math here? Or is any
        // 2^128-period Weyl-style sequence sufficient? (Currently just
        // SIMD addition for simplicity, unsure of the period).
        self.input += u64x2::new(1, 2);

        macro_rules! round {
            () => {{
                kk += KEY_WEYL;
                v = aes_enc(v, kk);
            }};
        }

        round!();
        round!();
        round!();
        round!();

        /*for _ in 0..5 - 1 {
            kk += KEY_WEYL;
            v = aes_enc(v, kk);
        }*/

        kk += KEY_WEYL;
        aes_enc_last(v, kk)
    }
}

impl SeedableRng for Ars5 {
    type Seed = [u8; 0];

    #[inline(always)]
    fn from_seed(_seed: Self::Seed) -> Self {
        unimplemented!()
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut seed_u64 = [u64x2::default(); 2];
        rng.try_fill(seed_u64.as_byte_slice_mut())?;

        Ok(Self {
            input: seed_u64[0],
            key: seed_u64[1],
        })
    }
}

/// ARS-7 from [Random123]
///
/// A single stream
///
/// > We favor use of the latter variants, with the extra safety margin, since
/// the performance penalty is quite small.
///
/// [Random123]: http://www.deshawresearch.com/resources_random123.html
pub struct Ars7 {
    input: u64x2,
    key: u64x2,
}

impl Ars7 {
    #[inline(always)]
    pub fn generate(&mut self) -> u64x2 {
        let mut kk = self.key;
        let mut v = self.input ^ kk;

        // Do we need exact u128 increment math here? Or is any
        // 2^128-period Weyl-style sequence sufficient? (Currently just
        // SIMD addition for simplicity, unsure of the period).
        self.input += u64x2::new(1, 2);

        macro_rules! round {
            () => {{
                kk += KEY_WEYL;
                v = aes_enc(v, kk);
            }};
        }

        round!();
        round!();
        round!();
        round!();
        round!();
        round!();

        /*for _ in 0..7 - 1 {
            kk += KEY_WEYL;
            v = aes_enc(v, kk);
        }*/

        kk += KEY_WEYL;
        aes_enc_last(v, kk)
    }
}

impl SeedableRng for Ars7 {
    type Seed = [u8; 0];

    #[inline(always)]
    fn from_seed(_seed: Self::Seed) -> Self {
        unimplemented!()
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut seed_u64 = [u64x2::default(); 2];
        rng.try_fill(seed_u64.as_byte_slice_mut())?;

        Ok(Self {
            input: seed_u64[0],
            key: seed_u64[1],
        })
    }
}
