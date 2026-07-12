# TurboCrc

Hardware accelerated implementation of CRC32C for computing 32-bit cyclic redundency check (CRC)

## Usage

Add following to your `Cargo.toml`,

```toml
[dependencies]
turbo_crc = { version = "0.0.3" }
```

## Benchmarks

Observed throughout on x86_64 using `sse4.2` ISA,

| Buffer Size | Throughput |
|:------------|:-----------|
| 64 KiB      | 8.65 GiB/s |
| 256 KiB     | 8.68 GiB/s |
| 1 MiB       | 8.47 GiB/s |
| 16 MiB      | 8.43 GiB/s |
| 64 MiB      | 8.39 GiB/s |

Observed throughout on aarch64 using `crc32cd` instruction,

| Buffer Size | Throughput  |
|:----------- |:------------|
| 64 KiB      | 11.92 GiB/s |
| 256 KiB     | 11.97 GiB/s |
| 1 MiB       | 11.99 GiB/s |
| 16 MiB      | 11.86 GiB/s |
| 64 MiB      | 11.84 GiB/s |

> [!NOTE]
> TL;DR: `turbo_crc` sustains ~8.5 GiB/sec on x86_64 and ~12 GiB/sec on aarch64 across buffers
> from 64 KiB to 64 MiB.

## Example

```rs
use turbo_crc::TurboCrc;

let buffer = vec![0x0Au8; 0x400 * 0x400 * 0x40];
let crc = TurboCrc::crc(&buffer);

assert_ne!(crc, 0);
```
