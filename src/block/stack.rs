use eyre::{eyre,ErrReport};
use std::collections::HashMap;
use std::str;

use crate::block::bitmap::Bitmap;
use crate::byte;

use super::background::Background;
use super::card::Card;
use super::font::Font;
use super::style::Style;

use super::data_layout::BlockLayoutGeneric as gen;
use super::data_layout::StackDataLayout as st;

use super::general::Block;

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
    format: StackFormat,

    backgrounds: Vec<&'a Background<'a>>,
    first_background: &'a Background<'a>,

    object: Vec<&'a Block<'a>>,
    first_card: &'a Card<'a>,

    // in order of appearance in the file format
    version: (u32, u32, u32, u32),

    checksum: u32,

    // (top, left, bottom, right)
    card_window_coords: (u16, u16, u16, u16),
    screen_coords: (u16, u16, u16, u16),

    coords: (u16, u16),

    fonts: &'a [&'a Font<'a>],
    styles: &'a [&'a Style<'a>],

    size: (u16, u16), // width, height

    script: &'a str
}

impl<'a> Stack<'a> {
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

        // version
        let (
            version_at_creation,
            version_at_last_compacting,
            version_at_last_modification_since_last_compacting,
            version_at_last_modification
        ) = (
            byte::u32_from_u8(&bytes[st::HyperCardVersionAtCreationStart()..st::HyperCardVersionAtCreationEnd()]),
            byte::u32_from_u8(&bytes[st::HyperCardVersionAtLastCompactingStart()..st::HyperCardVersionAtLastCompactingEnd()]),
            byte::u32_from_u8(&bytes[st::HyperCardVersionAtLastModificationSinceLastCompactingStart()..st::HyperCardVersionAtLastModificationSinceLastCompactingEnd()]),
            byte::u32_from_u8(&bytes[st::HyperCardVersionAtLastModificationStart()..st::HyperCardVersionAtLastModificationEnd()])
        );

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
        let mut master_table: HashMap<u8, u32> = HashMap::new();
        let master_table_raw = &bytes[offset..offset+(block_size as usize)/2];

        // store the master IDs
        for i in 0_u32..(block_size / 8) as u32 {
            let item = &master_table_raw[(i*4) as usize..((i+1)*4) as usize];
            // first 24 bits is the offset (as a multiple of 32). last 8 is block's "ID number"
            let location = byte::u24_from_u8(&item[0..3]) * 32;
            let id = item[3];
            // if the pointer is 0 then it's a "free block". we don't care about those, ignore them.
            if location == 0x00 {
                continue;
            }

            master_table.insert(id, location);
        }

        let mut objects: Vec<Block<'a>> = Vec::new();

        // loop through all the pointers we got and construct blocks off of them.
        for (id, location) in &master_table {
            let location = *location;
            let id = *id;

            let block_type = str::from_utf8(&bytes[location as usize+gen::BlockTypeStart()..location as usize+gen::BlockTypeEnd()])?;
            let block_size = byte::u32_from_u8(&bytes[location as usize+gen::BlockSizeStart()..location as usize+gen::BlockSizeEnd()]);
            let chunk = &bytes[location as usize..location as usize+block_size as usize];
            match block_type {
                "LIST" | "PAGE" => {
                    // redundant data that speeds up insertion/deletions in the original tools
                    // but we read these cards as read only and thus this is about as useful  as
                    // the master block
                },
                "BMAP" => {
                    let b = Bitmap::from(chunk).unwrap();
                    &b.image.save(format!("{}_{}.png",block_type,location));
                    objects.push(Block::Bitmap(b));

                }
                _ => {
                    println!("Unimplemented: block {} '{}' at {:#08x}",id,block_type,location);
                }
            }
        }

        Ok(())
    }
}