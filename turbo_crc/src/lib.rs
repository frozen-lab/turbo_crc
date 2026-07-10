//! Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundency check (CRC)
//!
//! ## Example
//!
//! ```
//! use turbo_crc::crc;
//!
//! let standard_vector = b"123456789";
//! assert_eq!(crc(standard_vector), 0xCBF43926);
//! ```

include!(concat!(env!("OUT_DIR"), "/table.rs"));

/// Compute a 32-bit crc for a given data buffer
///
/// ## Example
///
/// ```
/// use turbo_crc::crc;
///
/// let data = b"Hello, world!";
/// assert_ne!(crc(data), 0);
/// ```
#[inline(always)]
pub fn crc(buffer: &[u8]) -> u32 {
    let mut crc = !0;
    for &byte in buffer {
        let index = ((crc ^ (byte as u32)) & 0xFF) as usize;
        crc = (crc >> 8) ^ TABLE[index];
    }

    crc ^ !0u32
}
