use std::env;
use std::fs;
use std::io::{self, Write};

use sehx::sex_mut;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let mut bytes = fs::read(filename).unwrap();

    for mut byte in bytes.iter_mut() { 
        sex_mut(&mut byte);
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write(&bytes).unwrap();
}
