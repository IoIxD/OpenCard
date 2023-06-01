// Byte coercion: we need to be able to convert big endian to little endian
// because that's what these file formats are in

pub fn convert_u8_array_big_to_little(b: &[u8]) -> Vec<u8> {
    let mut fb = vec![0; b.len()];
    fb.copy_from_slice(b);
    for i in 0..fb.len() {
        fb[i] = b[i].reverse_bits();
    }
    return fb;
}

pub fn u64_from_u8(b: &[u8]) -> u64 {
    ((b[0] as u64) << 56)
        + ((b[1] as u64) << 48)
        + ((b[2] as u64) << 40)
        + ((b[3] as u64) << 32)
        + ((b[4] as u64) << 24)
        + ((b[5] as u64) << 16)
        + ((b[6] as u64) << 8)
        + ((b[7] as u64) << 0)
}

pub fn u32_from_u8(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

pub fn u24_from_u8(b: &[u8]) -> u32 {
    ((b[0] as u32) << 16) + ((b[1] as u32) << 8) + ((b[2] as u32) << 0)
}

pub fn u16_from_u8(b: &[u8]) -> u16 {
    u16::from_be_bytes([b[0], b[1]])
}

pub fn i64_from_u8(b: &[u8]) -> i64 {
    i64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

pub fn i32_from_u8(b: &[u8]) -> i32 {
    i32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

pub fn i24_from_u8(b: &[i8]) -> i32 {
    ((b[0] as i32) << 16) + ((b[1] as i32) << 8) + ((b[2] as i32) << 0)
}

pub fn i16_from_u8(b: &[u8]) -> i16 {
    i16::from_be_bytes([b[0], b[1]])
}

macro_rules! byte_range {
    (all, $arr:ident, $pkg:ident::$name:ident) => {
        paste::item! {
            &$arr[$pkg::[< $name Start >]()..$pkg::[< $name End >]()]
        }
    };
    (all, $arr:ident, $offset:stmt, $pkg:ident::$name:ident) => {
        paste::item! {
            &$arr[$offset+$pkg::[< $name Start >]()..$offset+$pkg::[< $name End >]()]
        }
    };
    ($ty:ident, $arr:ident, $pkg:ident::$name:ident) => {
        paste::item! {
            crate::byte::[< $ty _from_u8 >](&$arr[$pkg::[< $name Start >]()..$pkg::[< $name End >]()])
        }
    };
    ($ty:ident, $arr:ident, $offset:stmt, $pkg:ident::$name:ident) => {
        paste::item! {
            crate::byte::[< $ty _from_u8 >](&$arr[$offset+$pkg::[< $name Start >]()..$offset+$pkg::[< $name End >]()])
        }
    };
}

pub(crate) use byte_range;
