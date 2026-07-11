use turbo_crc::TurboCrc;

fn main() {
    let buffer = vec![0x0Au8; 0x400 * 0x400 * 0x40];
    let crc = std::hint::black_box(TurboCrc::crc(&buffer));

    std::hint::black_box(crc);
}
