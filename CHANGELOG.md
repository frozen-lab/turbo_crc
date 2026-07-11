# Changelog

## `0.0.2`

- Impl of slicing-by-16 crc computation
  - Improved public api from `turbo_crc::crc()` to `turbo_crc::TurboCrc::crc()`
  - 6x improved performance from `~0.5 GiB` to `~3 GiB`

## `0.0.1`

- Impl of `crc` function to compute crc w/ byte-to-byte method sustaining ~0.5 GiB throughput

## `0.0.0`

- Initial release
