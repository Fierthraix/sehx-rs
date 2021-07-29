pub type IH = i8;
pub type IA = i16;
pub type IB = i32;
pub type ID = i64;
pub type UH = u8;
pub type UA = u16;
pub type UB = u32;
pub type UD = u64;

pub fn sehx(x: &u8) -> u16 {
    let mut u: u16 = *x as u16;
    u = (u & 0x000f) | ((u & 0x00f0) << 4);
    u | 0x6060
}

pub fn sehxd(x: &u32) -> u64 {
    let mut u: u64 = *x as u64;
    u = (u & 0x000000000000ffff) | ((u & 0x00000000ffff0000) << 16);
    u = (u & 0x000000ff000000ff) | ((u & 0x0000ff000000ff00) << 8);
    u = (u & 0x000f000f000f000f) | ((u & 0x00f000f000f000f0) << 4);
    u | 0x6060606060606060
}

fn split_u16_to_u8s(x: &u16) -> impl IntoIterator<Item = u8> {
    [(*x & 0x00ff) as u8, ((*x & 0xff00) >> 8) as u8]
}

pub fn sehx_u8_buf(buf: &'_ [u8]) -> impl Iterator<Item = u8> + '_ {
    buf.iter().map(|byte| sehx(byte)).flat_map(|ua| split_u16_to_u8s(&ua))
}

pub fn sehx_vec(buf: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(buf.len() * 2);

    for byte in buf {
        result.extend(split_u16_to_u8s(&sehx(byte)));
    }

    result
}
