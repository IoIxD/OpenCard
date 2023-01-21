use crate::byte;
use crate::macroman::macroman_to_char;

use crate::byte::byte_range;

use eyre::{eyre,ErrReport};

use super::data_layout::PartLayout as p;
use super::data_layout::PartContentEntryLayout as pc;
use super::data_layout::PartContentEntryStyleLayout as st;

#[derive(Debug,Clone)]
pub struct Part {
    ty: PartType,

    position: (u16, u16, u16, u16),
    style: PartStyle,

    title_width: u16,
    text_alignment: TextAlignment,

    font_id: u16,
    font_size: u16,
    text_flags: u8,

    line_height: u16,
    name: String,
    script: String,

    contents: Vec<ContentEntry>,
}

#[derive(Debug,Clone)]
pub struct ContentEntry {
    id: u16,
    styles: Option<Vec<ContentEntryStyle>>,
    text: String,
}

#[derive(Debug,Clone)]
pub enum PartType {
    Button,
    Field,
    Unknown
}

#[derive(Debug,Clone)]
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
    Popup,
    Unknown
}

#[derive(Debug,Clone)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    ForceLeftAlign,
    ForceCenterAlign,
    ForceRightAlign,
    Unknown
}

#[derive(Debug,Clone)]
pub struct ContentEntryStyle {
    text_position: u16,
    id: u16,
}

impl Part {
    pub fn from(b: &[u8], part_content_num: u16, part_content_list_size: u32) -> Result<Self, ErrReport> {
        let ty = match byte_range!(u16, b, p::PartID) {
            0 => PartType::Button,
            1 => PartType::Field,
            _ => PartType::Unknown,
        };
        let position = (
            byte_range!(u16, b, p::PartRectTop),
            byte_range!(u16, b, p::PartRectLeft),
            byte_range!(u16, b, p::PartRectBottom),
            byte_range!(u16, b, p::PartRectRight)
        );

        let style = match &b[p::StyleStart()] {
            0 => PartStyle::Transparent,
            1 => PartStyle::Opaque,
            2 => PartStyle::Rectangle,
            3 => PartStyle::RoundRectangle,
            4 => PartStyle::Shadow,
            5 => PartStyle::Checkbox,
            6 => PartStyle::Radio,
            7 => PartStyle::Scrolling,
            8 => PartStyle::Standard,
            9 => PartStyle::Default,
            10 => PartStyle::Oval,
            11 => PartStyle::Popup,
            _ => PartStyle::Unknown,
        };

        let title_width = byte_range!(u16, b, p::TitleWidthOrLastSelectedLine);

        let text_alignment = match byte_range!(u16, b, p::TextAlignment) as i16 {
            0 => TextAlignment::Left,
            1 => TextAlignment::Center,
            -1 => TextAlignment::Right,
            -2 => TextAlignment::ForceLeftAlign,
            -3 => TextAlignment::ForceCenterAlign,
            -4 => TextAlignment::ForceRightAlign,
            _ => TextAlignment::Unknown,
        };

        let font_id = byte_range!(u16, b, p::TextFontID);
        let font_size = byte_range!(u16, b, p::TextSize);
        let text_flags = *&b[p::TextFlagsStart()];

        let line_height = byte_range!(u16, b, p::LineHeight);

        let mut offset = p::LineHeightEnd();
        // name and stack script, both terminated by nil
        let mut stack: Vec<char> = Vec::new();
        loop {
            if offset >= b.len() {
                return Err(eyre!("out of bounds"));
            }
            let ch = (&b)[offset];
            if ch == 0 {
                break;
            }
            stack.push(macroman_to_char(ch));
            offset += 1;
        }
        println!("\n");
        let name: String = (&stack).iter().collect();
        let mut stack: Vec<char> = Vec::new();
        loop {
            if offset >= b.len() {
                return Err(eyre!("out of bounds"));
            }
            let ch = (&b)[offset];
            if ch == 0 {
                break;
            }
            stack.push(macroman_to_char(ch));
            offset += 1;
        }
        let code: String = (&stack).iter().collect();

        let mut content_entries: Vec<ContentEntry> = Vec::new();

        // part content entry
        for _ in 0..part_content_num {
            let block = &b[offset..offset+part_content_list_size as usize];
            let id = byte_range!(u16, block, pc::PartID);

            let tmp = &b[pc::PlainTextMarkerOrStyleLengthByte1Start()];
            // if that first byte is 0 then there's no styles.
            let mut styles: Option<Vec<ContentEntryStyle>> = None;
            let mut noffset = offset;
            if *tmp != 0 {
                noffset += pc::StyleLengthByte2End();
                let mut tstyles: Vec<ContentEntryStyle> = Vec::new();
                // get the 'style length'; but negate the first bit because it's always set.
                let mut style_length = byte::u16_from_u8(&b[pc::PlainTextMarkerOrStyleLengthByte1Start()..pc::StyleLengthByte2End()]);
                style_length &= i16::MAX as u16;
                style_length /= 4;
                for _ in 0..style_length {
                    let styleblock = &b[noffset..noffset+0x04 as usize];
                    let text_position = byte_range!(u16, styleblock, st::TextPosition);
                    let id2 = byte_range!(u16, styleblock, st::StyleID);
                    tstyles.push(ContentEntryStyle { text_position, id: id2 });
                    noffset += 0x04 as usize;
                }
                styles = Some(tstyles);
            }
            // name, both terminated by nil
            let mut stack: Vec<char> = Vec::new();
            loop {
                let ch = (&b)[noffset];
                if ch == 0 {
                    break;
                }
                stack.push(macroman_to_char(ch)); // blindly unwrap because we can be certain it's a valid mac roman character
                noffset += 1;
            }
            let text: String = (&stack).iter().collect();
            content_entries.push(ContentEntry {
                id,
                styles,
                text,
            });
            offset += part_content_list_size as usize;
        }

        println!("\n===========");

        Ok(Part{
            ty,
            position,
            style,
            title_width,
            text_alignment,
            font_id,
            font_size,
            text_flags,
            line_height,
            name,
            script: code,
            contents: content_entries,
        })
    }
}