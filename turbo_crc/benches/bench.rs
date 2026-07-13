//! Benchmarks to measure througputs of crc32
//! Run using: `taskset -c 1,2,3 cargo bench --bench bench --profile bench`

const KB: usize = 0x400;
const MB: usize = KB * KB;

#[inline(always)]
fn make_buffer(len: usize) -> Vec<u8> {
    let mut x = 0x1234_5678u32;
    let mut out = vec![0u8; len];

    for byte in &mut out {
        x = x.wrapping_mul(0x19660D).wrapping_add(0x3C6EF35F);
        *byte = (x >> 0x18) as u8;
    }

    out
}

#[divan::bench(args = [
    0x10 * KB,
    0x40 * KB,
    0x100 * KB,
    1 * MB,
    0x10 * MB,
    0x40 * MB,
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
