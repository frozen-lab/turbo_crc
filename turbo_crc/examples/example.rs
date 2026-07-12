extern crate turbo_crc;

fn main() {
    let buffer = vec![0x0Au8; 0x400 * 0x400 * 0x40];
    let crc = std::hint::black_box(turbo_crc::crc32c(&buffer));

    std::hint::black_box(crc);
}
