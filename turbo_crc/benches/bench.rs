const KB: usize = 0x400;
const MB: usize = KB * KB;

#[divan::bench(args = [64 * KB, 256 * KB, 1 * MB, 16 * MB, 64 * MB, 128 * MB])]
fn crc_throughput(bencher: divan::Bencher, size: usize) {
    let data = vec![0x0Au8; size];

    bencher
        .counter(divan::counter::BytesCount::new(size))
        .bench(|| divan::black_box(turbo_crc::crc(&data)));
}

fn main() {
    divan::main();
}
