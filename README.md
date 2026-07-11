# TurboCrc

Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundency check (CRC)

## Usage

Add following to your `Cargo.toml`,

```toml
[dependencies]
turbo_crc = { version = "0.0.1" }
```

## Benchmarks

Observed throughput across multiple benchmark runs,

| Buffer Size | Throughput |
|:------------|:-----------|
| 64 KiB      | 523 MB/s   |
| 256 KiB     | 522 MB/s   |
| 1 MiB       | 521 MB/s   |
| 16 MiB      | 513 MB/s   |

TL;DR: Sustains ~0.5 GiB/s across buffers from 64 KiB to 64 MiB

## Example

```rs
use turbo_crc::crc;

let standard_vector = b"123456789";
assert_eq!(crc(standard_vector), 0xCBF43926);
```
