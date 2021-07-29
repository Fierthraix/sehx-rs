pub type IH = i8;
pub type IA = i16;
pub type IB = i32;
pub type ID = i64;
pub type UH = u8;
pub type UA = u16;
pub type UB = u32;
pub type UD = u64;

/// Convert a single `u8` to an expanded `u16` ASCII.
pub fn sehx(x: &u8) -> u16 {
    let mut u: u16 = *x as u16;
    u = (u & 0x000f) | ((u & 0x00f0) << 4);
    u | 0x6060
}

/// Convert a `u32` to an expanded `u64`.
pub fn sehxd(x: &u32) -> u64 {
    let mut u: u64 = *x as u64;
    u = (u & 0x000000000000ffff) | ((u & 0x00000000ffff0000) << 16);
    u = (u & 0x000000ff000000ff) | ((u & 0x0000ff000000ff00) << 8);
    u = (u & 0x000f000f000f000f) | ((u & 0x00f000f000f000f0) << 4);
    u | 0x6060606060606060
}

/// Split a `u16` into it's upper and lower bytes.
fn split_u16_to_u8s(x: &u16) -> impl IntoIterator<Item = u8> {
    [(x & 0x00ff) as u8, ((x & 0xff00) >> 8) as u8]
}

/// Convert a bytestream to it's expanded hexadecimal output format.
pub fn sehx_u8_buf(buf: &'_ [u8]) -> impl Iterator<Item = u8> + '_ {
    buf.iter().map(|byte| sehx(byte)).flat_map(|ua| split_u16_to_u8s(&ua))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    #[test]
    fn it_works() {
        let bin = fs::read("test/moto.bin").unwrap();
        let expected = fs::read("test/moto.sehx").unwrap();

        let result: Vec<u8> = sehx_u8_buf(&bin).collect();
        assert_eq!(result, expected);
    }
}
