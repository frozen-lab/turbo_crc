//! Hardware accelerated implementation of CRC32C for computing 32-bit Cyclic Redundency Check (CRC)

/// Hardware accelerated implementation of CRC32C for computing 32-bit Cyclic Redundency Check (CRC)
///
/// ## Example
///
/// ```
/// use turbo_crc::TurboCrc;
///
/// assert_eq!(std::mem::size_of::<TurboCrc>(), 0);
/// ```
pub struct TurboCrc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert_eq!(std::mem::size_of::<TurboCrc>(), 0);
    }
}
