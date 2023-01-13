use eyre::{eyre,ErrReport};
use std::str;

use crate::byte;

use super::background::Background;
use super::list::List;
use super::card::Card;
use super::font::Font;
use super::style::Style;

use crate::data_layout::BlockLayoutGeneric as gen;
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
        let block_type = str::from_utf8(name)?;
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

        println!("Loading a {:?} formatted stack",format);

        // backgrounds
        let backgrounds: Vec<Background> = Vec::new();
        let cards: Vec<Card> = Vec::new();

        // get any values that don't need to be malformed or "changed" later here, in the order they
        // appear in the file. this improves load times a bit on older hard drives.

        // misc. shit
        let password_hash = byte::u32_from_u8(&bytes[st::PasswordHashStart()..st::PasswordHashEnd()]);
        let protection_flags = byte::u16_from_u8(&bytes[st::ProtFlagsStart()..st::ProtFlagsEnd()]);
        let hypercard_version_at_creation = byte::u32_from_u8(&bytes[st::HyperCardVersionAtCreationStart()..st::HyperCardVersionAtCreationEnd()]);
        let hypercard_version_at_last_compacting = byte::u32_from_u8(&bytes[st::HyperCardVersionAtLastCompactingStart()..st::HyperCardVersionAtLastCompactingEnd()]);
        let hypercard_version_at_last_modification_since_last_compacting = byte::u32_from_u8(&bytes[st::HyperCardVersionAtLastModificationSinceLastCompactingStart()..st::HyperCardVersionAtLastModificationSinceLastCompactingEnd()]);
        let hypercard_version_at_last_modification = byte::u32_from_u8(&bytes[st::HyperCardVersionAtLastModificationStart()..st::HyperCardVersionAtLastModificationEnd()]);

        // positioning
        let win_top = byte::u16_from_u8(&bytes[st::CardWindowTopStart()..st::CardWindowTopEnd()]);
        let win_left = byte::u16_from_u8(&bytes[st::CardWindowLeftStart()..st::CardWindowLeftEnd()]);
        let win_bottom = byte::u16_from_u8(&bytes[st::CardWindowBottomStart()..st::CardWindowBottomEnd()]);
        let win_right = byte::u16_from_u8(&bytes[st::CardWindowRightStart()..st::CardWindowRightEnd()]);
        let scr_top = byte::u16_from_u8(&bytes[st::ScreenTopStart()..st::ScreenTopEnd()]);
        let scr_left = byte::u16_from_u8(&bytes[st::ScreenLeftStart()..st::ScreenLeftEnd()]);
        let scr_bottom = byte::u16_from_u8(&bytes[st::ScreenBottomStart()..st::ScreenBottomEnd()]);
        let scr_right = byte::u16_from_u8(&bytes[st::ScreenRightStart()..st::ScreenRightEnd()]);
        let x_coord = byte::u16_from_u8(&bytes[st::XCoordStart()..st::XCoordEnd()]);
        let y_coord = byte::u16_from_u8(&bytes[st::YCoordStart()..st::YCoordEnd()]);

        let width = byte::u16_from_u8(&bytes[st::WidthStart()..st::WidthEnd()]);
        let height = byte::u16_from_u8(&bytes[st::HeightStart()..st::HeightEnd()]);

        // tables
        let font_table: Vec<&Font> = Vec::new();
        let style_table: Vec<&Style> = Vec::new();

        // skip to 0x600 and get the stack script, which is terminated by 0x00
        let mut offset = 0x601;
        let mut stack: Vec<u8> = Vec::new();
        loop {
            let ch = (&bytes)[offset];
            if ch == 0 {
                break;
            }
            stack.push(ch);
            offset += 1;
        }
        let code = str::from_utf8(&stack);

        // skip to 0x800. we should see the master block.
        offset = 0x800;
        let block_type = str::from_utf8(&bytes[offset+gen::BlockTypeStart()..offset+gen::BlockTypeEnd()])?;
        if block_type != "MAST" {
            return Err(eyre!("Stack block was not followed up by a master block. Not continuing for fear of data corruption or an incompatible file."));
        }

        let block_size = byte::u32_from_u8(&bytes[offset+gen::BlockSizeStart()..offset+gen::BlockSizeEnd()]);

        offset += 0x20;

        // collect the table and parse it.
        let master_table = &bytes[offset..offset+(block_size as usize)/2];

        // it's in chunks of 32 bit integers.
        for i in 0_u32..(block_size / 8) as u32 {
            let item = &master_table[(i*4) as usize..((i+1)*4) as usize];
            // first 24 bits is the offset. last 8 is block's "ID number"
            let location = byte::u24_from_u8(&item[0..3]) * 32;
            let id = item[3];

            // if the pointer is 0 then it's a "free block". we don't care about those, ignore them.
            if(location == 0x00) {
                continue;
            }
            println!("Block {}: {:#08x}",id,location);
        }

        Ok(())
    }
}