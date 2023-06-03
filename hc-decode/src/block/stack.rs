use eyre::{eyre, ErrReport};
use futures::future::join_all;

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::str;

use crate::block::bitmap::Bitmap;
use crate::byte;
use crate::byte::byte_range;
use crate::macroman::macroman_to_char;

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

pub enum HyperCardVersionStatus {
    Release,
    Development(u32),
}

impl Display for HyperCardVersionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            HyperCardVersionStatus::Release => f.write_str("release"),
            HyperCardVersionStatus::Development(a) => f.write_fmt(format_args!("release {}", a)),
        }
    }
}

pub enum HyperCardVersionState {
    Final,
    Beta,
    Alpha,
    Development,
    Unknown(i32),
}

impl Display for HyperCardVersionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            HyperCardVersionState::Final => f.write_str("final"),
            HyperCardVersionState::Beta => f.write_str("beta"),
            HyperCardVersionState::Alpha => f.write_str("alpha"),
            HyperCardVersionState::Development => f.write_str("development"),
            HyperCardVersionState::Unknown(a) => f.write_fmt(format_args!("unknown({})", a)),
        }
    }
}

pub struct HypercardVersion(u32);
impl HypercardVersion {
    pub fn whole(&self) -> u32 {
        self.0
    }
    pub fn major(&self) -> u32 {
        format!("{:x}", (&self.0 >> 24) & 0xff).parse().unwrap()
    }
    pub fn minor(&self) -> f32 {
        format!("{:x}", (&self.0 >> 16) & 0xff)
            .parse::<f32>()
            .unwrap()
            / 100.0
    }
    pub fn state(&self) -> HyperCardVersionState {
        let state = format!("{:x}", (&self.0 >> 8) & 0xff).parse().unwrap();
        match state {
            80 => HyperCardVersionState::Final,
            60 => HyperCardVersionState::Beta,
            40 => HyperCardVersionState::Alpha,
            20 => HyperCardVersionState::Development,
            _ => HyperCardVersionState::Unknown(state),
        }
    }
    pub fn version_status(&self) -> HyperCardVersionStatus {
        let j = format!("{:x}", (&self.0 >> 0) & 0xff).parse().unwrap();
        if j == 0 {
            HyperCardVersionStatus::Release
        } else {
            HyperCardVersionStatus::Development(j)
        }
    }
}

impl Display for HypercardVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "version {} {} {}",
            self.major() as f32 + &self.minor(),
            &self.state(),
            &self.version_status()
        ))
    }
}

impl Debug for HypercardVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "version {} {} {}",
            self.major() as f32 + &self.minor(),
            &self.state(),
            &self.version_status()
        ))
    }
}

#[derive(Debug)]
pub struct Stack {
    pub format: StackFormat,

    pub backgrounds: Vec<Background>,
    pub first_background: Background,

    pub objects: HashMap<u32, Block>,

    pub cards: Vec<Card>,
    pub first_card: Card,

    // in order of appearance in the file format
    pub version: (
        HypercardVersion,
        HypercardVersion,
        HypercardVersion,
        HypercardVersion,
    ),

    /// (top, left, bottom, right)
    pub card_window_coords: (u16, u16, u16, u16),
    /// (top, left, bottom, right)
    pub screen_coords: (u16, u16, u16, u16),

    /// (x, y)
    pub coords: (u16, u16),

    pub fonts: Vec<Font>,
    pub styles: HashMap<u32, Style>,

    /// width, height
    pub size: (u16, u16),

    pub script: String,
}

impl Stack {
    pub async fn from(bytes: &[u8]) -> Result<Stack, ErrReport> {
        // if the size of the file isn't even 8 bytes, it's invalid.
        if bytes.len() < 8 {
            return Err(eyre!(
                "Provided file is not a valid HyperCard file; it's not even 8 bytes long"
            ));
        }

        // If it's a valid file we should see block type "STAK" at this position.
        // we can also check the checksum but the checksum is wrongly documented and also bullshit
        let name = &bytes[st::BlockTypeStart()..st::BlockTypeEnd()];
        let block_type = str::from_utf8(name)?;
        if block_type != "STAK" {
            return Err(eyre!(
                "Provided file is not a valid HyperCard file; Stack block not found."
            ));
        }

        // ...or was it?
        let checksum = bytes[0..0x5FF].iter().map(|f| *f as i32).sum::<i32>();
        println!("Checksum is {}", checksum);

        let stack_size = byte_range!(u32, bytes, st::BlockSize);

        let format_raw = byte_range!(u32, bytes, st::HyperCardFormat);

        let format = match format_raw {
            0 => StackFormat::NotHyperCard,
            1..=7 => StackFormat::PreReleaseHyperCard1x,
            8 => StackFormat::HyperCard1x,
            9 => StackFormat::PreReleaseHyperCard2x,
            10 => StackFormat::HyperCard2x,
            11..=u32::MAX => StackFormat::Unsupported,
        };

        println!("Loading a {:?} formatted stack", format);

        // backgrounds
        let backgrounds: Vec<Background> = Vec::new();
        let cards: Vec<Card> = Vec::new();

        let first_background_id = byte_range!(u32, bytes, st::FirstBackgroundID);
        let first_card_id = byte_range!(u32, bytes, st::FirstCardID);

        // get any values that don't need to be malformed or "changed" later here, in the order they
        // appear in the file. this improves load times a bit on older hard drives.

        // version
        println!("{:#x}", st::HyperCardVersionAtCreationEnd());
        let version_raw = (
            byte_range!(u32, bytes, st::HyperCardVersionAtCreation),
            byte_range!(u32, bytes, st::HyperCardVersionAtLastCompacting),
            byte_range!(
                u32,
                bytes,
                st::HyperCardVersionAtLastModificationSinceLastCompacting
            ),
            byte_range!(u32, bytes, st::HyperCardVersionAtLastModification),
        );
        let version = (
            HypercardVersion(version_raw.0),
            HypercardVersion(version_raw.1),
            HypercardVersion(version_raw.2),
            HypercardVersion(version_raw.3),
        );

        // positioning
        let card_window_coords = (
            byte_range!(u16, bytes, st::CardWindowTop),
            byte_range!(u16, bytes, st::CardWindowLeft),
            byte_range!(u16, bytes, st::CardWindowBottom),
            byte_range!(u16, bytes, st::CardWindowRight),
        );
        let screen_coords = (
            byte_range!(u16, bytes, st::ScreenTop),
            byte_range!(u16, bytes, st::ScreenLeft),
            byte_range!(u16, bytes, st::ScreenBottom),
            byte_range!(u16, bytes, st::ScreenRight),
        );
        let coords = (
            byte_range!(u16, bytes, st::XCoord),
            byte_range!(u16, bytes, st::YCoord),
        );

        let size = (
            byte_range!(u16, bytes, st::Width),
            byte_range!(u16, bytes, st::Height),
        );

        // tables
        let font_table: Vec<&Font> = Vec::new();
        let style_table: Vec<&Style> = Vec::new();

        // skip to 0x600 and get the stack script, which is terminated by 0x00
        let mut offset = 0x600;
        let mut stack: Vec<char> = Vec::new();
        loop {
            let ch = (&bytes)[offset];
            if ch == 0 {
                break;
            }
            stack.push(macroman_to_char(ch));
            offset += 1;
        }
        let script: String = (&stack).iter().collect();

        // set the offest to the nearest multiple of 4.
        let remainder = offset % 0x200;
        offset = offset + 0x200 - remainder;

        let block_type =
            str::from_utf8(&bytes[offset + gen::BlockTypeStart()..offset + gen::BlockTypeEnd()])?;
        /*if block_type != "MAST" {
            return Err(eyre!("Stack block was not followed up by a master block. Not continuing for fear of data corruption or an incompatible file."));
        }*/

        let block_size = byte_range!(u32, bytes, offset, gen::BlockSize);

        offset += 0x20;

        // collect the table and parse it.
        let mut master_table: HashMap<u8, u32> = HashMap::new();
        let master_table_raw = &bytes[offset..offset + (block_size as usize) / 2];

        // store the master IDs
        for i in 0_u32..(block_size / 8) as u32 {
            let item = &master_table_raw[(i * 4) as usize..((i + 1) * 4) as usize];
            // first 24 bits is the offset (as a multiple of 32). last 8 is block's "ID number"
            let location = byte::u24_from_u8(&item[0..3]) * 32;
            let id = item[3];
            // if the pointer is 0 then it's a "free block". we don't care about those, ignore them.
            if location == 0x00 {
                continue;
            }

            master_table.insert(id, location);
        }
        let mut futures = vec![];
        // loop through all the pointers we got and construct blocks off of them.
        // we construct futures that do this so that we can use multithreading
        for (id, location) in &master_table {
            let j = byte_range!(all, bytes, *location as usize, gen::BlockType);

            let block_type = match str::from_utf8(j) {
                Ok(a) => a,
                Err(err) => unsafe {
                    println!("Invalid block type '{}'", str::from_utf8_unchecked(j));
                    continue;
                },
            };
            let block_size = byte_range!(u32, bytes, *location as usize, gen::BlockSize);
            let chunk = &bytes[*location as usize..*location as usize + block_size as usize];
            futures.push(stack_parse(*location, *id, block_type.to_string(), chunk));
        }
        let objects: HashMap<u32, Block> = join_all(futures.into_iter())
            .await
            .into_iter()
            .filter(|f| f.is_some())
            .map(|f| f.unwrap())
            .collect();

        let j = objects.clone();

        // the final stretch

        // construct the backgrounds and cards.
        let (mut first_background, mut backgrounds) = (None, Vec::new());
        let (mut first_card, mut cards) = (None, Vec::new());
        let mut fonts = Vec::new();
        let mut styles = None;
        {
            (first_background, backgrounds) = filter_backgrounds(&objects, first_background_id);
        }
        {
            (first_card, cards) = filter_cards(&objects, first_card_id);
            fonts = filter_fonts(&objects);
            styles = match filter_styles(&objects) {
                Some(a) => Some(a),
                None => return Err(eyre!("No style table found.")),
            };
        }

        Ok(Stack {
            format,
            backgrounds,
            first_background: first_background.unwrap(),
            objects: j,
            cards,
            first_card: first_card.unwrap(),
            version,
            //checksum: todo!(),
            card_window_coords,
            screen_coords,
            coords,
            fonts,
            styles: styles.unwrap(),
            size,
            script,
        })
    }
}

fn filter_backgrounds(
    objects: &HashMap<u32, Block>,
    first_background_id: u32,
) -> (Option<Background>, Vec<Background>) {
    let mut first_background: Option<Background> = None;
    let backgrounds = objects
        .into_iter()
        .filter(|f| f.1.is_background())
        .map(|f| {
            if *f.0 == first_background_id {
                first_background = Some(f.1.get_background());
            }
            f.1.get_background()
        })
        .collect();
    (first_background, backgrounds)
}
fn filter_cards(objects: &HashMap<u32, Block>, first_card_id: u32) -> (Option<Card>, Vec<Card>) {
    let mut first_card: Option<Card> = None;
    let cards = objects
        .into_iter()
        .filter(|f| f.1.is_card())
        .map(|f| {
            if *f.0 == first_card_id {
                first_card = Some(f.1.get_card());
            }
            f.1.get_card()
        })
        .collect();
    (first_card, cards)
}
fn filter_fonts(objects: &HashMap<u32, Block>) -> Vec<Font> {
    objects
        .into_iter()
        .filter(|f| f.1.is_font())
        .map(|f| f.1.get_font())
        .collect()
}
fn filter_styles(objects: &HashMap<u32, Block>) -> Option<HashMap<u32, Style>> {
    for obj in objects {
        if obj.1.is_style() {
            return Some(obj.1.get_style());
        }
    }
    None
}

async fn stack_parse(
    location: u32,
    id: u8,
    block_type: String,
    chunk: &[u8],
) -> Option<(u32, Block)> {
    let block_id = byte_range!(u32, chunk, gen::BlockID);
    match block_type.as_str() {
        "LIST" | "PAGE" => {
            // redundant data that speeds up insertion/deletions in the original tools
            // but we read these cards as read only and thus this is about as useful  as
            // the master block
            None
        }
        "BMAP" => {
            let b = Bitmap::from(chunk).unwrap_or_else(|f| {
                panic!("error parsing bitmap\n{}", f);
            });
            Some((block_id, Block::Bitmap(b)))
        }
        "CARD" => {
            let c = Card::from(chunk).unwrap_or_else(|f| {
                panic!("error parsing card\n{}", f);
            });
            Some((block_id, Block::Card(c)))
        }
        "STBL" => {
            let s = Style::vec_from(chunk).unwrap_or_else(|f| {
                panic!("error parsing style\n{}", f);
            });
            Some((block_id, Block::Style(s)))
        }
        "BKGD" => {
            let b = Background::from(chunk).unwrap_or_else(|f| {
                panic!("error parsing background\n{}", f);
            });
            Some((block_id, Block::Background(b)))
        }
        _ => {
            println!(
                "Unimplemented: block {} '{}' at {:#08x}",
                id, block_type, location
            );
            None
        }
    }
}
