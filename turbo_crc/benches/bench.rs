/// Benchmarks to observe througput of crc
/// Run using: `taskset -c 1 cargo bench --bench bench --profile bench`

const KB: usize = 0x400;
const MB: usize = KB * KB;

#[divan::bench(args = [64 * KB, 256 * KB, 1 * MB, 16 * MB, 64 * MB])]
fn crc_throughput(bencher: divan::Bencher, size: usize) {
    let data = vec![(size & 0xFF) as u8; size];

    bencher
        .counter(divan::counter::BytesCount::new(size))
        .bench(|| divan::black_box(turbo_crc::crc(&data)));
}

fn main() {
    divan::main();
}
