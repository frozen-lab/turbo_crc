//! Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundancy check (CRC)
//!
//! ## Benchmark
//!
//! Observed throughout on x86_64 using `sse4.2` ISA,
//!
//! | Buffer Size | Throughput |
//! |:------------|-----------:|
//! | 64 KiB      | 8.65 GiB/s |
//! | 256 KiB     | 8.68 GiB/s |
//! | 1 MiB       | 8.47 GiB/s |
//! | 16 MiB      | 8.43 GiB/s |
//! | 64 MiB      | 8.39 GiB/s |
//!
//! Observed throughout on aarch64 using `crc32cd` instruction,
//!
//! | Buffer Size | Throughput |
//! |:----------- | ----------:|
//! | 64 KiB      | 11.92 GiB/s |
//! | 256 KiB     | 11.97 GiB/s |
//! | 1 MiB       | 11.99 GiB/s |
//! | 16 MiB      | 11.86 GiB/s |
//! | 64 MiB      | 11.84 GiB/s |
//!
//! > [!NOTE]
//! > TL;DR: `turbo_crc` sustains ~8.5 GiB/sec on x86_64 and ~12 GiB/sec on aarch64 across buffers
//! > from 64 KiB to 64 MiB.
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

    let mut crc = !0u32;
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

    !crc
}
