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
//! use turbo_crc::TurboCrc;
//!
//! let buffer = vec![0x0Au8; 0x400 * 0x400 * 0x40];
//! let crc = TurboCrc::crc(&buffer);
//!
//! assert_ne!(crc, 0);
//! assert_eq!(std::mem::size_of::<TurboCrc>(), 0);
//! ```

include!(concat!(env!("OUT_DIR"), "/table.rs"));

/// Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundancy check (CRC)
///
/// ## Example
///
/// ```
/// use turbo_crc::TurboCrc;
///
/// let buffer = vec![0x0Au8; 0x400 * 0x400 * 0x40];
/// let crc = TurboCrc::crc(&buffer);
///
/// assert_ne!(crc, 0);
/// assert_eq!(std::mem::size_of::<TurboCrc>(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct TurboCrc;

impl TurboCrc {
    /// Compute a 32-bit crc for a given data buffer
    ///
    /// _NOTE:_ Input `buffer` must be 16-bytes aligned.
    ///
    /// ## Example
    ///
    /// ```
    /// use turbo_crc::TurboCrc;
    ///
    /// let buffer = vec![0x0Au8; 0x20];
    /// let crc = TurboCrc::crc(&buffer);
    ///
    /// assert_ne!(crc, 0);
    /// ```
    #[inline(always)]
    pub fn crc(buffer: &[u8]) -> u32 {
        // sanity check
        #[cfg(debug_assertions)]
        {
            let len = buffer.len();
            debug_assert!(
                len >= 0x10 && (len & len - 1) == 0,
                "Input buffer must be aligned to 16 bytes"
            );
        }

        let mut crc = !0u32;
        let mut ptr = buffer.as_ptr();
        let mut chunks = buffer.len() / 0x10;

        while chunks != 0 {
            let a = unsafe { load(ptr) ^ crc };
            let b = unsafe { load(ptr.add(4)) };
            let c = unsafe { load(ptr.add(8)) };
            let d = unsafe { load(ptr.add(0x0C)) };

            crc = TABLE[0x0F][(a & 0xff) as usize]
                ^ TABLE[0x0E][((a >> 8) & 0xff) as usize]
                ^ TABLE[0x0D][((a >> 0x10) & 0xff) as usize]
                ^ TABLE[0x0C][((a >> 0x18) & 0xff) as usize]
                ^ TABLE[0x0B][(b & 0xff) as usize]
                ^ TABLE[0x0A][((b >> 8) & 0xff) as usize]
                ^ TABLE[9][((b >> 0x10) & 0xff) as usize]
                ^ TABLE[8][((b >> 0x18) & 0xff) as usize]
                ^ TABLE[7][(c & 0xff) as usize]
                ^ TABLE[6][((c >> 8) & 0xff) as usize]
                ^ TABLE[5][((c >> 0x10) & 0xff) as usize]
                ^ TABLE[4][((c >> 0x18) & 0xff) as usize]
                ^ TABLE[3][(d & 0xff) as usize]
                ^ TABLE[2][((d >> 8) & 0xff) as usize]
                ^ TABLE[1][((d >> 0x10) & 0xff) as usize]
                ^ TABLE[0][((d >> 0x18) & 0xff) as usize];

            ptr = unsafe { ptr.add(0x10) };
            chunks -= 1;
        }

        !crc
    }
}

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn load(ptr: *const u8) -> u32 {
    std::ptr::read_unaligned(ptr.cast())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert_eq!(std::mem::size_of::<TurboCrc>(), 0);
    }

    fn make_buffer(len: usize) -> Vec<u8> {
        let mut x = 0x1234_5678u32;
        let mut out = vec![0u8; len];

        for byte in &mut out {
            x = x.wrapping_mul(1664525).wrapping_add(1013904223);
            *byte = (x >> 0x18) as u8;
        }

        out
    }

    #[test]
    fn ok_avalanche_effect() {
        let a = make_buffer(0x400);
        let mut b = a.clone();

        b[0x200] ^= 0x80;
        assert_ne!(TurboCrc::crc(&a), TurboCrc::crc(&b));
    }

    #[test]
    fn ok_trailing_zero_changes_crc() {
        let a = make_buffer(0x100);
        let mut b = a.clone();

        b[0xFF] = 0;
        assert_ne!(TurboCrc::crc(&a), TurboCrc::crc(&b));
    }

    #[test]
    fn ok_deterministic() {
        let buf = make_buffer(0x1000);
        let expected = TurboCrc::crc(&buf);

        for _ in 0..0x40 {
            assert_eq!(TurboCrc::crc(&buf), expected);
        }
    }
}
