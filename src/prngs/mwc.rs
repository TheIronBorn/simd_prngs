use std::arch::x86_64::*;
use std::simd::*;

use rng_impl::*;

pub struct Mwc8 {
    buffer: [u64x2; 4],
    idx: u8,
}

impl Mwc8 {
    #[inline(always)]
    pub fn generate(&mut self) -> u64x2 {
        // Factors for multiply-with-carry
        const FACTORS: [u64x2; 4] = [
            u64x2::new(4294963023, 3947008974),
            u64x2::new(4162943475, 2654432763),
            u64x2::new(3874257210, 2936881968),
            u64x2::new(4294957665, 2811536238),
        ];

        let x = self.buffer[self.idx as usize];
        let f = FACTORS[self.idx as usize];

        // widening multiply the low 32 bits of each lane
        let mut y = u64x2::from_bits(unsafe {
            _mm_mul_epu32(__m128i::from_bits(x), __m128i::from_bits(f))
        });

        y += x >> 32; // add old carry
        self.buffer[self.idx as usize] = y; // new x and carry

        // The uppermost bits of the carry are not sufficiently random. Randomize some more for output
        y ^= y << 30; // output function
        y ^= y >> 35;
        y ^= y << 13;

        self.idx = (self.idx + 1) % self.buffer.len() as u8;
        y
    }
}

impl SeedableRng for Mwc8 {
    type Seed = [u8; 0];

    #[inline(always)]
    fn from_seed(_seed: Self::Seed) -> Self {
        unimplemented!()
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut seed = [u64x2::default(); 4];
        rng.try_fill(seed.as_byte_slice_mut())?;

        // seeding is very complicated. I'm hoping improper seeding won't
        // affect speed

        Ok(Self {
            buffer: seed,
            idx: 0,
        })
    }
}

pub struct Mwc2 {
    state: u64x2,
}

impl Mwc2 {
    #[inline(always)]
    pub fn generate(&mut self) -> u64x2 {
        // Factors for multiply-with-carry
        const FACTORS: u64x2 = u64x2::new(4294963023, 3947008974);

        // widening multiply the low 32 bits of each lane
        let mut y = u64x2::from_bits(unsafe {
            _mm_mul_epu32(__m128i::from_bits(self.state), __m128i::from_bits(FACTORS))
        });

        y += self.state >> 32; // add old carry
        self.state = y; // new x and carry

        // The uppermost bits of the carry are not sufficiently random. Randomize some more for output
        y ^= y << 30; // output function
        y ^= y >> 35;
        y ^= y << 13;

        y
    }
}

impl SeedableRng for Mwc2 {
    type Seed = [u8; 0];

    #[inline(always)]
    fn from_seed(_seed: Self::Seed) -> Self {
        unimplemented!()
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut seed = [u64x2::default(); 1];
        rng.try_fill(seed.as_byte_slice_mut())?;

        // seeding is very complicated. I'm hoping improper seeding won't
        // affect speed

        Ok(Self { state: seed[0] })
    }
}

pub struct Mwc4 {
    buffer: [u64x2; 2],
    idx: bool,
}

impl Mwc4 {
    #[inline(always)]
    pub fn generate(&mut self) -> u64x2 {
        // Factors for multiply-with-carry
        const FACTORS: [u64x2; 2] = [
            u64x2::new(4294963023, 3947008974),
            u64x2::new(4162943475, 2654432763),
        ];

        let x = self.buffer[self.idx as usize];
        let f = FACTORS[self.idx as usize];

        // widening multiply the low 32 bits of each lane
        let mut y = u64x2::from_bits(unsafe {
            _mm_mul_epu32(__m128i::from_bits(x), __m128i::from_bits(f))
        });

        y += x >> 32; // add old carry
        self.buffer[self.idx as usize] = y; // new x and carry

        // The uppermost bits of the carry are not sufficiently random. Randomize some more for output
        y ^= y << 30; // output function
        y ^= y >> 35;
        y ^= y << 13;

        self.idx = !self.idx;
        y
    }
}

impl SeedableRng for Mwc4 {
    type Seed = [u8; 0];

    #[inline(always)]
    fn from_seed(_seed: Self::Seed) -> Self {
        unimplemented!()
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut seed = [u64x2::default(); 2];
        rng.try_fill(seed.as_byte_slice_mut())?;

        // seeding is very complicated. I'm hoping improper seeding won't
        // affect speed

        Ok(Self {
            buffer: seed,
            idx: false,
        })
    }
}
