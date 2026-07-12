//! Benchmarks to measure througput of crc
//! Run using: `taskset -c 1 cargo bench --bench bench --profile bench`

const KB: usize = 0x400;
const MB: usize = KB * KB;

extern crate turbo_crc;

#[divan::bench(args = [0x40 * KB, 0x100 * KB, 1 * MB, 0x10 * MB, 0x40 * MB])]
fn crc_throughput(bencher: divan::Bencher, size: usize) {
    let data = vec![(size & 0xFF) as u8; size];

    bencher
        .counter(divan::counter::BytesCount::new(size))
        .bench(|| divan::black_box(turbo_crc::crc32c(&data)));
}

fn main() {
    divan::main();
}
