use std::env;
use std::fs;
use std::io::{self, Write};

use sehx::{sehx, sehx_to_u8};

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let bytes = fs::read(filename).unwrap();
    let sehx_bytes: Vec<u16> = bytes.iter().map(|byte| sehx(byte)).collect();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write(&sehx_to_u8(&sehx_bytes)).unwrap();
}
