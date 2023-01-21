use std::collections::HashMap;

use eyre::ErrReport;
use crate::byte::byte_range;

use super::data_layout::StyleTableLayout as st;
use super::data_layout::StyleLayout as s;

#[derive(Debug,Clone)]
pub struct Style {
    font: i16,
    style_flags: i16,
    font_size: i16,
}

impl Style {
    pub fn vec_from(b: &[u8]) -> Result<HashMap<u32, Self>, ErrReport> {
        let style_num = byte_range!(u32, b, st::StyleNum);
        let offset = st::NextStyleIDEnd();
        let mut styles: HashMap<u32, Self> = HashMap::new();
        for _ in 0..style_num {
            let chunk = &b[offset..offset+23];
            let id = byte_range!(u32, chunk, s::StyleID);
            let font = byte_range!(u16, chunk, s::FontID) as i16;
            let style_flags = byte_range!(u16, chunk, s::StyleFlags) as i16;
            let font_size = byte_range!(u16, chunk, s::FontSize) as i16;
            styles.insert(id, Style { font, style_flags, font_size });
        }
        Ok(styles)
    }
}