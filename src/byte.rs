// Byte coercion: we need to be able to convert big endian to little endian
// because that's what these file formats are in

pub fn convert_u8_array_big_to_little(b: &[u8]) -> Vec<u8> {
    let mut fb = vec![0; b.len()];
    fb.copy_from_slice(b);
    for i in 0..fb.len() {
        fb[i] = b[i].reverse_bits();
    };
    return fb;
}

pub fn u64_from_u8(b: &[u8]) -> u64 {
    ((b[0] as u64) << 56) +
    ((b[1] as u64) << 48) +
    ((b[2] as u64) << 40) +
    ((b[3] as u64) << 32) +
    ((b[4] as u64) << 24) +
    ((b[5] as u64) << 16) +
    ((b[6] as u64) <<  8) +
    ((b[7] as u64) <<  0)
}

pub fn u32_from_u8(b: &[u8]) -> u32 {
    ((b[0] as u32) << 24) +
    ((b[1] as u32) << 16) +
    ((b[2] as u32) <<  8) +
    ((b[3] as u32) <<  0)
}

pub fn u16_from_u8(b: &[u8]) -> u16 {
    ((b[0] as u16) <<  8) +
    ((b[1] as u16) <<  0)
}