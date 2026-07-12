//! Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundancy check (CRC)
//!
//! ## Benchmark
//!
//! Observed throughput across multiple benchmark runs,
//!
//! | Buffer Size | Throughput |
//! |:------------|-----------:|
//! | 64 KiB      | 2.89 GiB/s |
//! | 256 KiB     | 2.89 GiB/s |
//! | 1 MiB       | 2.89 GiB/s |
//! | 16 MiB      | 3.07 GiB/s |
//! | 64 MiB      | 3.08 GiB/s |
//!
//! TL;DR: Sustains ~3 GiB/s across buffers from 64 KiB to 64 MiB
//!
//! ## Example
//!
//! ```
//! extern crate turbo_crc;
//!
//! let standard_vector = b"12345678";
//! assert_eq!(turbo_crc::crc32c(standard_vector), 0x6087809A);
//! ```

/// Compute a hardware accelerated 32-bit crc for a given data buffer using `sse4.2` ISA
///
/// ## Example
///
/// ```
/// extern crate turbo_crc;
///
/// let standard_vector = b"12345678";
/// assert_eq!(turbo_crc::crc32c(standard_vector), 0x6087809A);
/// ```
#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn crc32c(buffer: &[u8]) -> u32 {
    // sanity check
    debug_assert!(buffer.len() & 7 == 0, "Input buffer must be 8 bytes aligned");

    let mut crc = (!0u32) as u64;
    let mut len = buffer.len();
    let mut ptr = buffer.as_ptr();

    while len > 0 {
        unsafe {
            let qword = core::ptr::read_unaligned(ptr as *const u64);
            crc = core::arch::x86_64::_mm_crc32_u64(crc, qword);

            ptr = ptr.add(8);
            len -= 8;
        }
    }

    (!crc) as u32
}

/// Compute a hardware accelerated 32-bit crc for a given data buffer using ArmV8 `crc` instruction
///
/// ## Example
///
/// ```
/// extern crate turbo_crc;
///
/// let standard_vector = b"12345678";
/// assert_eq!(turbo_crc::crc32c(standard_vector), 0x6087809A);
/// ```
#[inline(always)]
#[cfg(target_arch = "aarch64")]
pub fn crc32c(buffer: &[u8]) -> u32 {
    // sanity check
    debug_assert!(buffer.len() & 7 == 0, "Input buffer must be 8 bytes aligned");

    let mut crc = (!0u32) as u64;
    let mut len = buffer.len();
    let mut ptr = buffer.as_ptr();

    while len > 0 {
        unsafe {
            let qword = core::ptr::read_unaligned(ptr as *const u64);
            crc = core::arch::aarch64::__crc32cd(crc, qword);

            ptr = ptr.add(8);
            len -= 8;
        }
    }

    (!crc) as u32
}
