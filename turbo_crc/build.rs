use std::{env, fs, path};

/// Polynomial in refelected form (little endian) over `0x1EDC6F41`
///
/// Used by CRC32C algorithm as initilization value
const CASTAGNOLI_POLYNOMIAL: u32 = 0x82F63B78;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("read env var w/ OUT_DIR");
    let destination = path::Path::new(&out_dir).join("table.rs");

    let mut table = vec![0u32; 0x100];
    for i in 0..0x100 {
        table[i] = calculate_crc(i as u32);
    }

    let mut src = String::new();
    src.push_str("const BYTE_BY_BYTE_TABLE: [u32; 0x100] = [\n");

    for value in table {
        src.push_str(&format!("    0x{:08X},\n", value));
    }

    src.push_str("];\n");
    fs::write(destination, src).expect("write to table.rs");
}

#[inline]
fn calculate_crc(mut input: u32) -> u32 {
    for _ in 0..8 {
        if input & 1 == 1 {
            input = (input >> 1) ^ CASTAGNOLI_POLYNOMIAL;
        } else {
            input >>= 1;
        }
    }

    input
}
