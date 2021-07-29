use std::env;
use std::fs;
use std::io::{self, Write};

use sehx::sehx_vec;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let bytes = fs::read(filename).unwrap();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle
        //.write(&sehx_u8_buf(&bytes).collect::<Vec<u8>>())
        .write(&sehx_vec(&bytes))
        .unwrap();
}
