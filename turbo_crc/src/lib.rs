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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_test_standard_ieee_vector() {
        let data = b"123456789";
        assert_eq!(crc(data), 0xCBF43926);
    }

    #[test]
    fn ok_test_empty_buffer() {
        let data = b"";
        assert_eq!(crc(data), 0x00000000);
    }

    #[test]
    fn ok_test_single_byte() {
        let data = b"a";
        assert_eq!(crc(data), 0xE8B7BE43);
    }

    #[test]
    fn ok_test_long_pangram() {
        let data = b"The quick brown fox jumps over the lazy dog";
        assert_eq!(crc(data), 0x414FA339);
    }

    #[test]
    fn ok_test_avalanche_effect() {
        let data1 = b"Hello, world!";
        let data2 = b"Hello, world.";

        assert_ne!(crc(data1), crc(data2), "A single byte change must alter the CRC");
    }

    #[test]
    fn ok_test_trailing_zeros_mutation() {
        let data_base = b"secret_file";
        let data_padded = b"secret_file\0\0\0\0";

        assert_ne!(crc(data_base), crc(data_padded), "Trailing zeros must alter the CRC");
    }
}
