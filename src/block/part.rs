use super::font::Font;
use super::style::Style;

pub struct Part<'a> {
    id: u16,
    ty: PartType,
    hash_flags: u8,
    top: u16,
    left: u16,
    bottom: u16,
    right: u16,
    unk_flags: u8,
    style: PartStyle,

    title_width: u16,
    icon_id: u16,
    text_alignment: TextAlignment,

    font: &'a Font<'a>,
    font_size: u16,
    text_flags: u8,

    line_height: u8,
    name: &'a str,
    script: &'a str,

    contents: &'a [&'a ContentEntry<'a>],
}

pub struct ContentEntry<'a> {
    id: u16,
    styles: &'a[&'a Style<'a>],
    text: &'a str,
}

pub enum PartType {
    Button,
    Field
}

pub enum PartStyle {
    Transparent,
    Opaque,
    Rectangle,
    RoundRectangle,
    Shadow,
    Checkbox,
    Radio,
    Scrolling,
    Standard,
    Default,
    Oval,
    Popup
}

pub enum TextAlignment {
    Left,
    Center,
    Right,
    ForceLeftAlign,
    ForceCenterAlign,
    ForceRightAlign,
}
