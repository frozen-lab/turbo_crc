//! Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundancy check (CRC)
//!
//! ## Benchmark
//!
//! Observed throughput on x86_64 using `sse4.2` ISA,
//!
//! | Buffer Size | Throughput |
//! |:------------|-----------:|
//! | 64 KiB      | 8.65 GiB/s |
//! | 256 KiB     | 8.68 GiB/s |
//! | 1 MiB       | 8.47 GiB/s |
//! | 16 MiB      | 8.43 GiB/s |
//! | 64 MiB      | 8.39 GiB/s |
//!
//! Observed throughput on aarch64 using `crc32cd` instruction,
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

include!(concat!(env!("OUT_DIR"), "/table.rs"));

/// Computes a 32-bit cyclic redundancy check (CRC) using Castagnoli polynomial while leveraging
/// best available hardware instructions on x86 architectures
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
/// let standard_vector = b"12345678";
/// assert_eq!(turbo_crc::crc32c(standard_vector), 0x6087809A);
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
    let mut crc = (!0u32) as u64;

    let chunks = buffer.chunks_exact(8);
    let remaining_bytes = chunks.remainder();

    for chunk in chunks {
        unsafe {
            let qword = core::ptr::read_unaligned(chunk.as_ptr() as *const u64);
            crc = core::arch::x86_64::_mm_crc32_u64(crc, qword);
        }
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
    let mut crc = !0u32;

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
