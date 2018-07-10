//! The ChaCha random number generator.
//!
//! https://cr.yp.to/chacha.html

use std::simd::*;

use rng_impl::*;
use rand_core::le;

#[derive(Clone, Copy)]
struct ChaChaState {
    a: u32x4,
    b: u32x4,
    c: u32x4,
    d: u32x4
}

/// 4 rounds of ChaCha
///
/// A single stream
/// Memory: 512 bits
/// Not cryptographically strong but still has good statistical quality:
/// https://crypto.stackexchange.com/a/57672
pub struct ChaCha4 {
    state: ChaChaState,
}

impl ChaCha4 {
    #[inline(always)]
    pub fn generate(&mut self) -> u32x16 {
        fn core(state: ChaChaState) -> ChaChaState {
            let mut tmp = state;

            macro_rules! round {
                () => {{
                    // The `16` and `8` rotates could be replaced with shuffles
                    tmp.a += tmp.b; tmp.d = (tmp.d ^ tmp.a).rotate_left(16);
                    tmp.c += tmp.d; tmp.b = (tmp.b ^ tmp.c).rotate_left(12);
                    tmp.a += tmp.b; tmp.d = (tmp.d ^ tmp.a).rotate_left(8);
                    tmp.c += tmp.d; tmp.b = (tmp.b ^ tmp.c).rotate_left(7);

                    tmp.b = unsafe { simd_shuffle4(tmp.b, tmp.b, [1, 2, 3, 0]) };
                    tmp.c = unsafe { simd_shuffle4(tmp.c, tmp.c, [2, 3, 0, 1]) };
                    tmp.d = unsafe { simd_shuffle4(tmp.d, tmp.d, [3, 0, 1, 2]) };
                }}
            }

            round!();
            round!();
            round!();
            round!();

            tmp.a += state.a;
            tmp.b += state.b;
            tmp.c += state.c;
            tmp.d += state.d;

            tmp
        }

        let result = core(self.state);

        // update 64-bit counter, could probably be better
        self.state.d += u32x4::new(1, 0, 0, 0);
        // if d[0] == 0 { d[1] += 1; };
        let cmp = self.state.d.eq(u32x4::splat(0)).extract(0);
        self.state.d += cmp as u32 & u32x4::new(0, 1, 0, 0);

        // compiles to a few move/insert128 instructions
        let ab: u32x8 = unsafe { simd_shuffle8(result.a, result.b, [0, 1, 2, 3, 4, 5, 6, 7]) };
        let cd: u32x8 = unsafe { simd_shuffle8(result.c, result.d, [0, 1, 2, 3, 4, 5, 6, 7]) };
        unsafe { simd_shuffle16(ab, cd, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }
    }
}

impl SeedableRng for ChaCha4 {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_le = [0u32; 8];
        le::read_u32_into(&seed, &mut seed_le);
        Self {
            state: ChaChaState {
                a: u32x4::new(0x61707865, 0x3320646E, 0x79622D32, 0x6B206574), // constants
                b: u32x4::new(seed_le[0], seed_le[1], seed_le[2], seed_le[3]), // seed
                c: u32x4::new(seed_le[4], seed_le[5], seed_le[6], seed_le[7]), // seed
                d: u32x4::new(0, 0, 0, 0), // counter
            },
        }
    }
}
