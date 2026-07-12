# Changelog

## `0.0.3`

- (rs) Impl of hardware optimized crc32c computation
  - Impl of `crc32c` using `sse4.2` ISA on x86_64 w/ `~8.5 GiB per second` 
  - Impl of `crc32c` using `crc32cd` instruction on aarch64 (ArmV8.1 >=) w/ `~12 GiB per second`

## `0.0.2`

- (rs) Impl of slicing-by-16 crc computation
  - Improved public api from `turbo_crc::crc()` to `turbo_crc::TurboCrc::crc()`
  - 6x improved performance from `~0.5 GiB` to `~3 GiB`

## `0.0.1`

- (rs) Impl of `crc` function to compute crc w/ byte-to-byte method sustaining ~0.5 GiB throughput

## `0.0.0`

- Initial release
