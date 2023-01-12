use super::font::Font;

pub struct Style<'a> {
    id: i32,
    font: &'a Font<'a>,
    style_flags: u16,
    font_size: u16,
}