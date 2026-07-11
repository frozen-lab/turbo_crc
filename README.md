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
| 64 KiB      | 2.89 GiB/s |
| 256 KiB     | 2.89 GiB/s |
| 1 MiB       | 2.89 GiB/s |
| 16 MiB      | 3.07 GiB/s |
| 64 MiB      | 3.08 GiB/s |

TL;DR: Sustains ~3 GiB/s across buffers from 64 KiB to 64 MiB

## Example

```rs
use turbo_crc::TurboCrc;

let buffer = vec![0x0Au8; 0x400 * 0x400 * 0x40];
let crc = TurboCrc::crc(&buffer);

assert_ne!(crc, 0);
```
