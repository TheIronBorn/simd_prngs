use std::arch::x86_64::*;
use std::{mem, ptr};

use rng_impl::*;

/// AESRand, a counter-based invertible PRNG.
///
/// - Source: <https://github.com/dragontamer/AESRand>
/// - State: 128-bits
/// - Output: 256-bits
/// - Cycle Length: 2<sup>64</sup>
/// - Noncryptographic
/// - PractRand: >8TB
/// - BigCrush: passed
///
/// Good throughput, decent latency. Easily the best throughput of any 128-bit
/// PRNG in this library, and better than many 256-bit PRNGs.
///
/// Requires x86 AES support.
pub struct AesRand {
    state: __m128i,
    buffer: [__m128i; 2],
    full: bool,
}

impl AesRand {
    #[inline(always)]
    pub fn gen_array(&mut self) -> [__m128i; 2] {
        unsafe {
            #[rustfmt::skip]
            let increment = _mm_set_epi8(
                0x2f, 0x2b, 0x29, 0x25, 0x1f, 0x1d, 0x17, 0x13,
                0x11, 0x0D, 0x0B, 0x07, 0x05, 0x03, 0x02, 0x01,
            );

            self.state = _mm_add_epi64(self.state, increment);
            let penultimate = _mm_aesenc_si128(self.state, increment);
            let penultimate1 = _mm_aesenc_si128(penultimate, increment);
            let penultimate2 = _mm_aesdec_si128(penultimate, increment);
            [penultimate1, penultimate2]
        }
    }

    #[inline(always)]
    fn fill_buffer(&mut self) {
        self.buffer = self.gen_array();
        self.full = true;
    }
}

impl SimdRng for AesRand {
    type Result = __m128i;

    #[inline(always)]
    fn generate(&mut self) -> __m128i {
        if self.full {
            self.full = false;
            self.buffer[0]
        } else {
            let result = self.buffer[1];
            self.fill_buffer();
            result
        }
    }
}

impl RngCore for AesRand {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        unsafe { _mm_extract_epi32(self.generate(), 0) as u32 }
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        unsafe { _mm_extract_epi64(self.generate(), 0) as u64 }
    }

    // Custom implementation to best use the two outputs.
    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        const CHUNK_SIZE: usize = mem::size_of::<__m128i>();

        for large_chunk in dest.chunks_exact_mut(CHUNK_SIZE * 2) {
            for (ch, &res) in large_chunk
                .chunks_exact_mut(CHUNK_SIZE)
                .zip(&self.gen_array())
            {
                #[allow(clippy::cast_ptr_alignment)]
                let ptr = ch.as_mut_ptr() as *mut __m128i;
                unsafe {
                    _mm_storeu_si128(ptr, res);
                }
            }
        }

        let large_remainder = dest.chunks_exact_mut(CHUNK_SIZE * 2).into_remainder();

        for chunk in large_remainder.chunks_exact_mut(CHUNK_SIZE) {
            let results = self.generate();
            #[allow(clippy::cast_ptr_alignment)]
            let ptr = chunk.as_mut_ptr() as *mut __m128i;
            unsafe {
                _mm_storeu_si128(ptr, results);
            }
        }

        let remainder = large_remainder
            .chunks_exact_mut(CHUNK_SIZE)
            .into_remainder();
        if !remainder.is_empty() {
            let results = self.generate();
            let src_ptr = &results as *const __m128i as *const u8;
            let dst_ptr = remainder.as_mut_ptr();
            unsafe {
                ptr::copy_nonoverlapping(src_ptr, dst_ptr, remainder.len());
            }
        }
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for AesRand {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        #[allow(clippy::cast_ptr_alignment)]
        let ptr = seed.as_ptr() as *const __m128i;
        let mut rng = Self {
            state: unsafe { _mm_loadu_si128(ptr) },
            buffer: [unsafe { _mm_setzero_si128() }; 2],
            full: false,
        };
        rng.fill_buffer();
        rng
    }
}
