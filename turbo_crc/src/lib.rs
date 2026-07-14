//! Hardware accelerated implementation for computing 32-bit cyclic redundancy checks (CRC) using
//! the Castagnoli polynomial over the finite field GF(2)
//!
//! ## Benchmarks
//!
//! Measured peak throughput on x86_64 using the `sse4.2` CRC instruction,
//!
//! | Buffer Size | Throughput |
//! |:------------|:-----------|
//! | 16 KiB      | 9.52 GiB/s |
//! | 64 KiB      | 9.58 GiB/s |
//! | 256 KiB     | 9.43 GiB/s |
//! | 1 MiB       | 9.37 GiB/s |
//! | 16 MiB      | 8.33 GiB/s |
//! | 64 MiB      | 8.16 GiB/s |
//!
//! Measured peak throughput on aarch64 using the `crc32cd` instruction,
//!
//! | Buffer Size | Throughput  |
//! |:------------|:------------|
//! | 64 KiB      | 11.92 GiB/s |
//! | 256 KiB     | 11.97 GiB/s |
//! | 1 MiB       | 11.99 GiB/s |
//! | 16 MiB      | 11.86 GiB/s |
//! | 64 MiB      | 11.84 GiB/s |
//!
//! **TL;DR:** `turbo_crc` achieves up to `~9.5 GiB/s` on x86_64 and `~12.5 GiB/s` on aarch64 using
//! hardware-accelerated CRC instructions.
//!
//! ## Example
//!
//! ```
//! extern crate turbo_crc;
//!
//! // NOTE: Official CRC-32C (Castagnoli) check vector from RFC 3720
//! assert_eq!(turbo_crc::crc32c(b"123456789"), 0xE3069283);
//! ```

#![allow(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/table.rs"));

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const INITIAL_CRC: u32 = !0;

/// Computes a 32-bit cyclic redundancy check (CRC) using Castagnoli polynomial while leveraging
/// best available hardware instructions on x86 architectures
///
/// ## Example
///
/// ```
/// extern crate turbo_crc;
///
/// // NOTE: Official CRC-32C (Castagnoli) check vector from RFC 3720
/// assert_eq!(turbo_crc::crc32c(b"123456789"), 0xE3069283);
/// ```
#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn crc32c(buffer: &[u8]) -> u32 {
    if buffer.len() >= 0x40 {
        return unsafe { hw_sse_clmul_crc32c(buffer) };
    }

    hw_sse42_crc32(buffer)
}

/// Computes a 32-bit cyclic redundancy check (CRC) using Castagnoli polynomial while leveraging
/// best available hardware instructions on arm architectures
///
/// ## Example
///
/// ```
/// extern crate turbo_crc;
///
/// // NOTE: Official CRC-32C (Castagnoli) check vector from RFC 3720
/// assert_eq!(turbo_crc::crc32c(b"123456789"), 0xE3069283);
/// ```
#[inline(always)]
#[cfg(target_arch = "aarch64")]
pub fn crc32c(buffer: &[u8]) -> u32 {
    hw_armv81_crc32cd(buffer)
}

/// Computes a 32-bit cyclic redundancy check (CRC) using Castagnoli polynomial using built in
/// hardware instruction available on `sse4.2` ISA on x86_64 architecture
#[inline(always)]
#[cfg(target_arch = "x86_64")]
fn hw_sse42_crc32(buffer: &[u8]) -> u32 {
    let mut crc = u64::from(INITIAL_CRC);

    let chunks = buffer.chunks_exact(8);
    let remaining_bytes = chunks.remainder();

    for chunk in chunks {
        unsafe {
            let qword = core::ptr::read_unaligned(chunk.as_ptr() as *const u64);
            crc = core::arch::x86_64::_mm_crc32_u64(crc, qword);
        }
    }

    if remaining_bytes.is_empty() {
        return !(crc as u32);
    }

    let final_crc = sw_b2b_crc32(crc as u32, remaining_bytes);
    !final_crc
}

/// Computes a 32-bit cyclic redundancy check (CRC) using Castagnoli polynomial using the built in
/// hardware instruction `crc32cd` available when the optional CRC extension is present (mandatory
/// in Armv8.1-A and later)
#[inline(always)]
#[cfg(target_arch = "aarch64")]
fn hw_armv81_crc32cd(buffer: &[u8]) -> u32 {
    let mut crc = INITIAL_CRC;

    let chunks = buffer.chunks_exact(8);
    let remaining_bytes = chunks.remainder();

    for chunk in chunks {
        unsafe {
            let qword = core::ptr::read_unaligned(chunk.as_ptr() as *const u64);
            crc = core::arch::aarch64::__crc32cd(crc, qword);
        }
    }

    crc = sw_b2b_crc32(crc, remaining_bytes);
    !crc
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
unsafe fn hw_sse_clmul_crc32c(buffer: &[u8]) -> u32 {
    let k1_k2 = _mm_set_epi64x(0x1c6e41596, 0x154442bd4); // k2', k1'
    let k3_k4 = _mm_set_epi64x(0x0ccaa009e, 0x1751997d0); // k4', k3'
    let k5_k6 = _mm_set_epi64x(0x1db710640, 0x163cd6124); // k6', k5'
    let pl_mu = _mm_set_epi64x(0x1f7011641, 0x1db710641); // mu', p'

    let len = buffer.len();
    let mut ptr = buffer.as_ptr();

    let mut x3 = _mm_loadu_si128(ptr.add(0x00) as *const __m128i);
    let mut x2 = _mm_loadu_si128(ptr.add(0x10) as *const __m128i);
    let mut x1 = _mm_loadu_si128(ptr.add(0x20) as *const __m128i);
    let mut x0 = _mm_loadu_si128(ptr.add(0x30) as *const __m128i);

    let init_crc_vec = _mm_set_epi32(0, 0, 0, INITIAL_CRC as i32);
    x3 = _mm_xor_si128(x3, init_crc_vec);

    ptr = ptr.add(0x40);
    let mut remaining = len - 0x40;

    while remaining >= 0x40 {
        let n3 = _mm_loadu_si128(ptr.add(0x00) as *const __m128i);
        let n2 = _mm_loadu_si128(ptr.add(0x10) as *const __m128i);
        let n1 = _mm_loadu_si128(ptr.add(0x20) as *const __m128i);
        let n0 = _mm_loadu_si128(ptr.add(0x30) as *const __m128i);

        x3 = _mm_xor_si128(
            _mm_xor_si128(
                _mm_clmulepi64_si128::<0x00>(x3, k1_k2),
                _mm_clmulepi64_si128::<0x11>(x3, k1_k2),
            ),
            n3,
        );
        x2 = _mm_xor_si128(
            _mm_xor_si128(
                _mm_clmulepi64_si128::<0x00>(x2, k1_k2),
                _mm_clmulepi64_si128::<0x11>(x2, k1_k2),
            ),
            n2,
        );
        x1 = _mm_xor_si128(
            _mm_xor_si128(
                _mm_clmulepi64_si128::<0x00>(x1, k1_k2),
                _mm_clmulepi64_si128::<0x11>(x1, k1_k2),
            ),
            n1,
        );
        x0 = _mm_xor_si128(
            _mm_xor_si128(
                _mm_clmulepi64_si128::<0x00>(x0, k1_k2),
                _mm_clmulepi64_si128::<0x11>(x0, k1_k2),
            ),
            n0,
        );

        ptr = ptr.add(0x40);
        remaining -= 0x40;
    }

    let fold_x3 = _mm_xor_si128(
        _mm_clmulepi64_si128::<0x00>(x3, k3_k4),
        _mm_clmulepi64_si128::<0x11>(x3, k3_k4),
    );
    x2 = _mm_xor_si128(x2, fold_x3);

    let fold_x2 = _mm_xor_si128(
        _mm_clmulepi64_si128::<0x00>(x2, k3_k4),
        _mm_clmulepi64_si128::<0x11>(x2, k3_k4),
    );
    x1 = _mm_xor_si128(x1, fold_x2);

    let fold_x1 = _mm_xor_si128(
        _mm_clmulepi64_si128::<0x00>(x1, k3_k4),
        _mm_clmulepi64_si128::<0x11>(x1, k3_k4),
    );
    let mut state = _mm_xor_si128(x0, fold_x1);

    while remaining >= 0x10 {
        let next_slice = _mm_loadu_si128(ptr as *const __m128i);
        state = _mm_xor_si128(
            _mm_xor_si128(
                _mm_clmulepi64_si128::<0x00>(state, k3_k4),
                _mm_clmulepi64_si128::<0x11>(state, k3_k4),
            ),
            next_slice,
        );
        ptr = ptr.add(0x10);
        remaining -= 0x10;
    }

    let p_k5 = _mm_clmulepi64_si128::<0x10>(state, k5_k6); // Multiply state lower 64-bits with k5
    let state_shifted_8 = _mm_bsrli_si128::<8>(state);
    let state_96 = _mm_xor_si128(p_k5, state_shifted_8);

    let p_k6 = _mm_clmulepi64_si128::<0x00>(state_96, k5_k6); // Multiply state_96 lower 64-bits with k6
    let state_shifted_4 = _mm_bsrli_si128::<4>(state_96);
    let state_64 = _mm_xor_si128(p_k6, state_shifted_4);

    let quotient = _mm_clmulepi64_si128::<0x10>(state_64, pl_mu); // Multiply state_64 low 64-bits by mu'
    let quotient_poly = _mm_clmulepi64_si128::<0x00>(quotient, pl_mu); // Multiply quotient low 64-bits by p'

    let remainder = _mm_xor_si128(state_64, quotient_poly);
    let crc = _mm_extract_epi32::<1>(remainder) as u32;

    if remaining == 0 {
        return !crc;
    }

    let remaining_bytes = core::slice::from_raw_parts(ptr, remaining);
    let final_crc = sw_b2b_crc32(crc, remaining_bytes);
    !final_crc
}

/// Computes a 32-bit cyclic redundancy check (CRC) using Castagnoli polynomial for buffer of
/// an arbitrary length/size
#[inline(always)]
fn sw_b2b_crc32(mut crc: u32, buffer: &[u8]) -> u32 {
    for &byte in buffer {
        let index = ((crc ^ (byte as u32)) & 0xFF) as usize;
        crc = (crc >> 8) ^ BYTE_BY_BYTE_TABLE[index];
    }

    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    mod public {
        use super::*;

        #[test]
        fn ok_empty() {
            assert_eq!(crc32c(b""), 0x00000000);
        }

        #[test]
        fn ok_deterministic_check() {
            let buffer = vec![0xA5u8; 0x400 * 0x400];
            assert_eq!(crc32c(&buffer), crc32c(&buffer));
        }

        #[test]
        fn ok_tail_lengths() {
            for len in 0..0x10 {
                let buffer: Vec<u8> = (0..len).map(|x| x as u8).collect();
                assert_eq!(crc32c(&buffer), crc32c(&buffer));
            }
        }

        #[test]
        fn ok_various_lengths() {
            for len in 0..0x80 {
                let buffer: Vec<u8> = (0..len).map(|i| (i * 0x11) as u8).collect();
                assert_eq!(crc32c(&buffer), crc32c(&buffer));
            }
        }

        #[cfg(target_arch = "x86_64")]
        #[test]
        fn x86_backend_is_compiled() {
            let _ = hw_sse42_crc32(b"hello");
        }

        #[cfg(target_arch = "aarch64")]
        #[test]
        fn arm_backend_is_compiled() {
            let _ = hw_armv81_crc32cd(b"hello");
        }

        /// compare impl w/ official CRC-32C (source -> RFC 3720 Appendix B.4)
        mod rfc_3720 {
            use super::*;

            #[test]
            fn ok_standard_vector() {
                assert_eq!(crc32c(b"123456789"), 0xE3069283);
            }

            #[test]
            fn ok_single_byte() {
                assert_eq!(crc32c(b"a"), 0xC1D04330);
            }

            #[test]
            fn ok_two_bytes() {
                assert_eq!(crc32c(b"ab"), 0xE2A22936);
            }

            #[test]
            fn ok_three_bytes() {
                assert_eq!(crc32c(b"abc"), 0x364B3FB7);
            }

            #[test]
            fn ok_four_bytes() {
                assert_eq!(crc32c(b"abcd"), 0x92C80A31);
            }

            #[test]
            fn ok_eight_bytes() {
                assert_eq!(crc32c(b"12345678"), 0x6087809A);
            }
        }
    }

    mod software {
        use super::*;

        mod byte_to_byte {
            use super::*;

            #[test]
            fn ok_empty() {
                assert_eq!(!sw_b2b_crc32(!0, b""), 0x00000000);
            }

            #[test]
            fn ok_various_lengths() {
                for len in 0..0x100 {
                    let buffer: Vec<u8> = (0..len).map(|i| (i * 0x11) as u8).collect();
                    assert_eq!(!sw_b2b_crc32(!0, &buffer), crc32c(&buffer),);
                }
            }

            mod rfc_3720 {
                use super::*;

                #[test]
                fn ok_standard_vector() {
                    assert_eq!(!sw_b2b_crc32(!0, b"123456789"), 0xE3069283);
                }
            }
        }
    }

    mod hardware {
        use super::*;

        #[cfg(target_arch = "x86_64")]
        mod x86_64 {
            use super::*;

            #[test]
            fn ok_empty() {
                assert_eq!(hw_sse42_crc32(b""), 0x00000000);
            }

            #[test]
            fn ok_all_tail_lengths() {
                for tail in 0..8 {
                    let len = 0x40 + tail;
                    let buffer: Vec<u8> = (0..len).map(|i| (i * 0x0D) as u8).collect();

                    assert_eq!(hw_sse42_crc32(&buffer), crc32c(&buffer),);
                }
            }

            mod rfc_3720 {
                use super::*;

                #[test]
                fn ok_standard_vector() {
                    assert_eq!(hw_sse42_crc32(b"123456789"), 0xE3069283);
                }
            }
        }

        #[cfg(target_arch = "aarch64")]
        mod aarch64 {
            use super::*;

            #[test]
            fn ok_empty() {
                assert_eq!(hw_armv81_crc32cd(b""), 0x00000000);
            }

            #[test]
            fn ok_all_tail_lengths() {
                for tail in 0..8 {
                    let len = 0x40 + tail;
                    let buffer: Vec<u8> = (0..len).map(|i| (i * 0x0D) as u8).collect();

                    assert_eq!(hw_armv81_crc32cd(&buffer), crc32c(&buffer),);
                }
            }

            mod rfc_3720 {
                use super::*;

                #[test]
                fn ok_standard_vector() {
                    assert_eq!(hw_armv81_crc32cd(b"123456789"), 0xE3069283);
                }
            }
        }
    }
}
