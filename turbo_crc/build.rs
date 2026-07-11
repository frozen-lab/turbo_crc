use std::{env, fs, path};

/// mathematically chosen key from `https://datatracker.ietf.org/doc/html/rfc2083`
const DIVISOR: u32 = 0xEDB88320;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = path::Path::new(&out_dir).join("table.rs");

    let mut table = [0u32; 0x100];
    for i in 0..0x100 {
        table[i] = calculate_crc(i as u32);
    }

    let mut src = String::new();
    src.push_str("const TABLE: [u32; 256] = [\n");

    for value in table {
        src.push_str(&format!("    0x{:08X},\n", value));
    }

    src.push_str("];\n");
    fs::write(dest, src).unwrap();
}

fn calculate_crc(i: u32) -> u32 {
    let mut byte = i;
    for _ in 0..8 {
        if byte & 1 == 1 {
            byte = (byte >> 1) ^ DIVISOR;
        } else {
            byte >>= 1;
        }
    }

    byte
}
