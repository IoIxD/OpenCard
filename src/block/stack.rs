use eyre::{eyre,ErrReport};
use std::str;

use crate::byte;

use super::background::Background;
use super::list::List;
use super::card::Card;
use super::font::Font;
use super::style::Style;

use crate::data_layout::StackDataLayout as st;

#[derive(Debug)]
pub enum StackFormat {
    NotHyperCard,
    PreReleaseHyperCard1x,
    HyperCard1x,
    PreReleaseHyperCard2x,
    HyperCard2x,
    Unsupported,
}

pub struct Stack<'a> {
    size: u32,
    format: StackFormat,
    data_fork_size: u32,
    stack_size: u32,
    unk1: u32,
    max_of_prev_value: u32,     // wat

    background_num: u32,
    first_background: &'a Background<'a>,

    card_num: u32,
    first_card: &'a Card<'a>,

    list: &'a List<'a>,

    password_hash: u32,

    user_level: u16,

    prot_flags: u16,

    version_at_creation: u32,
    version_at_last_compacting: u32,
    version_at_last_modification_since_compacting: u32,
    version_at_last_modification: u32,

    checksum: u32,

    marked_card_num: u32,
    top_of_card_window: u16,
    left_of_card_window: u16,
    bottom_of_card_window: u16,
    right_of_card_window: u16,
    top_of_screen: u16,
    left_of_screen: u16,
    bottom_of_screen: u16,
    right_of_screen: u16,
    x_coord_scroll: u16,
    y_coord_scroll: u16,

    unk2: u16,

    font_table: &'a [&'a Font<'a>],
    style_table: &'a [&'a Style<'a>],

    height: u16,
    width: u16,

    unk3: u16,
    unk4: u16,

    patterns: &'a [u8],
    script: &'a str
}

impl Stack<'_> {
    pub fn from(bytes: &[u8]) -> Result<(), ErrReport> {
        // todo: ...huh.
        // let bytes = byte::convert_u8_array_big_to_little(bytes);

        // if the size of the file isn't even 8 bytes, it's invalid.
        if bytes.len() < 8 {
            return Err(eyre!("Provided file is not a valid HyperCard file; it's not even 8 bytes long"));
        }

        // If it's a valid file we should see block type "STAK" at this position.
        let name = &bytes[st::BlockTypeStart()..st::BlockTypeEnd()];
        let block_type = match str::from_utf8(name) {
            Ok(a) => a,
            Err(err) => {
                return Err(ErrReport::from(err));
            }
        };
        if block_type != "STAK" {
            return Err(eyre!("Provided file is not a valid HyperCard file; Stack block not found."));
        }

        let stack_size = byte::u32_from_u8(&bytes[st::BlockSizeStart()..st::BlockSizeEnd()]);

        let format_raw = byte::u32_from_u8(&bytes[st::HyperCardFormatStart()..st::HyperCardFormatEnd()]);

        let format = match format_raw {
            0 => StackFormat::NotHyperCard,
            1..=7 => StackFormat::PreReleaseHyperCard1x,
            8 => StackFormat::HyperCard1x,
            9 => StackFormat::PreReleaseHyperCard2x,
            10 => StackFormat::HyperCard2x,
            11..=u32::MAX => StackFormat::Unsupported,
        };

        println!("Loading a {:?} format file",format);

        // size/allocatin values
        
        Ok(())
    }
}