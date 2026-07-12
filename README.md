# TurboCrc

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://app.codspeed.io/frozen-lab/turbo_crc?utm_source=badge)

Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundency check (CRC)

## Usage

Add following to your `Cargo.toml`,

```toml
[dependencies]
turbo_crc = { version = "0.0.2" }
```

## Benchmarks

Observed throughput across multiple benchmark runs,

| Buffer Size | Throughput |
|:------------|-----------:|
| 64 KiB      | 8.65 GiB/s |
| 256 KiB     | 8.68 GiB/s |
| 1 MiB       | 8.47 GiB/s |
| 16 MiB      | 8.43 GiB/s |
| 64 MiB      | 8.39 GiB/s |

TL;DR: Sustains ~8.5 GiB/s across buffers from 64 KiB to 64 MiB

## Example

```rs
use turbo_crc::TurboCrc;

let buffer = vec![0x0Au8; 0x400 * 0x400 * 0x40];
let crc = TurboCrc::crc(&buffer);

assert_ne!(crc, 0);
```
