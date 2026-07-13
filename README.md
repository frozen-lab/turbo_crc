# TurboCrc

Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundency check (CRC)

## Usage

Add following to your `Cargo.toml`,

```toml
[dependencies]
turbo_crc = { version = "0.0.3" }
```

## Benchmarks

Measured peak throughput on x86_64 using the `sse4.2` CRC instruction,

| Buffer Size | Throughput |
|:------------|:-----------|
| 16 KiB      | 9.52 GiB/s |
| 64 KiB      | 9.58 GiB/s |
| 256 KiB     | 9.43 GiB/s |
| 1 MiB       | 9.37 GiB/s |
| 16 MiB      | 8.33 GiB/s |
| 64 MiB      | 8.16 GiB/s |

Measured peak throughput on aarch64 using the `crc32cd` instruction,

| Buffer Size | Throughput  |
|:------------|:------------|
| 64 KiB      | 11.92 GiB/s |
| 256 KiB     | 11.97 GiB/s |
| 1 MiB       | 11.99 GiB/s |
| 16 MiB      | 11.86 GiB/s |
| 64 MiB      | 11.84 GiB/s |

**TL;DR:** `turbo_crc` achieves up to `~9.5 GiB/s` on x86_64 and `~12.5 GiB/s` on aarch64 using
hardware-accelerated CRC instructions.

## Example

```rs
use turbo_crc::crc32c;

// NOTE: Official CRC-32C (Castagnoli) check vector from RFC 3720
assert_eq!(crc32c(b"123456789"), 0xE3069283);
``` 
