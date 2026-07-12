//! Benchmarks to measure througput of crc
//! Run using: `taskset -c 1 cargo bench --bench bench --profile release`

const KB: usize = 0x400;
const MB: usize = KB * KB;

extern crate turbo_crc;

#[inline(always)]
fn make_buffer(len: usize) -> Vec<u8> {
    let mut x = 0x1234_5678u32;
    let mut out = vec![0u8; len];

    for byte in &mut out {
        x = x.wrapping_mul(1664525).wrapping_add(1013904223);
        *byte = (x >> 0x18) as u8;
    }

    out
}

#[divan::bench(args = [
    1 * KB,
    0x10 * KB,
    0x40 * KB,
    0x100 * KB,
    0x200 * KB,
    1 * MB,
    0x10 * MB,
    0x64 * MB,
    0x100 * MB,
    0x200 * MB,
])]
fn crc_throughput(bencher: divan::Bencher, size: usize) {
    let buffer = make_buffer(size);

    bencher
        .counter(divan::counter::BytesCount::new(size))
        .bench(|| divan::black_box(turbo_crc::crc32c(&buffer)));
}

fn main() {
    divan::main();
}
