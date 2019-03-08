#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::mem;

use packed_simd::*;
use rand::RngCore;

pub trait SimdRng: RngCore {
    type Result: WriteBytes;

    fn generate(&mut self) -> Self::Result;

    fn generate_u32(&mut self) -> u32 {
        let mut bytes = [0; 4];
        self.fill_bytes_aligned(&mut bytes);
        u32::from_ne_bytes(bytes)
    }

    fn generate_u64(&mut self) -> u64 {
        let mut bytes = [0; 8];
        self.fill_bytes_aligned(&mut bytes);
        u64::from_ne_bytes(bytes)
    }

    fn fill_bytes_unaligned(&mut self, dest: &mut [u8]) {
        let chunk_size = mem::size_of::<Self::Result>();

        for chunk in dest.chunks_exact_mut(chunk_size) {
            self.generate().write_bytes_unaligned(chunk);
        }

        let remainder = dest.chunks_exact_mut(chunk_size).into_remainder();
        if !remainder.is_empty() {
            self.generate().write_few_bytes(remainder);
        }
    }

    fn fill_bytes_aligned(&mut self, dest: &mut [u8]) {
        let chunk_size = mem::size_of::<Self::Result>();

        for chunk in dest.chunks_exact_mut(chunk_size) {
            self.generate().write_bytes_aligned(chunk);
        }

        let remainder = dest.chunks_exact_mut(chunk_size).into_remainder();
        if !remainder.is_empty() {
            self.generate().write_few_bytes(remainder);
        }
    }
}

pub trait WriteBytes {
    fn write_bytes_unaligned(self, dest: &mut [u8]);
    fn write_bytes_aligned(self, dest: &mut [u8]);
    fn write_few_bytes(self, dest: &mut [u8]);
}

macro_rules! impl_write_bytes {
    ($u8xN:ident => $($ty:ty),+) => (
        $(
            impl WriteBytes for $ty {
                #[inline]
                fn write_bytes_unaligned(self, dest: &mut [u8]) {
                    $u8xN::from_bits(self).write_to_slice_unaligned(dest)
                }

                #[inline]
                fn write_bytes_aligned(self, dest: &mut [u8]) {
                    $u8xN::from_bits(self).write_to_slice_aligned(dest)
                }

                #[inline]
                fn write_few_bytes(self, dest: &mut [u8]) {
                    let mut buf = [0; mem::size_of::<Self>()];
                    self.write_bytes_unaligned(&mut buf);
                    let len = dest.len();
                    dest.copy_from_slice(&buf[..len]);
                }
            }
        )+
    );
}

impl_write_bytes! { u8x4 => u16x2 }
impl_write_bytes! { u8x8 => u16x4, u32x2 }
impl_write_bytes! { u8x16 => u16x8, u32x4, u64x2 }
impl_write_bytes! { u8x32 => u16x16, u32x8, u64x4 }
impl_write_bytes! { u8x64 => u16x32, u32x16, u64x8 }

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_write_bytes! { u8x16 => __m128i }
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_write_bytes! { u8x32 => __m256i }
