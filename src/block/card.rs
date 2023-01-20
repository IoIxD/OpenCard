use std::error::Error;

use super::part::Part;
use super::data_layout::CardLayout as c;
use crate::byte::{self, byte_range};
use crate::macroman::macroman_to_char;

pub struct Card {
    id: u32,
    bitmap_block_id: u32,
    flags: u16,
    parts: Vec<Part>,

    name: String,
    script: String,
}

impl Card {
    pub fn from(b: &[u8]) -> Result<Self, Box<dyn Error>> {
        /*for i in 0..b.len() {
            println!("{:#09x}\t\t{}",i,b[i]);
        }*/
        let id = byte_range!(u32,b,c::BlockID);
        let bitmap_block_id = byte_range!(u32,b,c::BitmapID);
        let flags = byte_range!(u16,b,c::Flags);
        let parent_page_id = byte_range!(u32, b, c::ParentPageID);
        let background_id = byte_range!(u32, b, c::BackgroundID);
        let part_num = byte_range!(u16, b, c::PartNum);
        let part_list_size = byte_range!(u32, b, c::PartListSize);
        let part_content_num = byte_range!(u16, b, c::PartContentNum);
        let part_content_list_size = byte_range!(u32, b, c::PartContentListSize);

        let mut offset = c::PartContentListSizeEnd();
        let mut old_offset = offset;
        let mut parts: Vec<Part> = Vec::new();
        let mut part_size = 0;
        for i in 0..part_num {
            //println!("{:#09x}",offset);
            // i don't know
            if offset + 0x02 >= parts.len() {
                continue
            }
            part_size = byte::u16_from_u8(&b[offset..offset + 0x02]);
            //println!("{}",part_size);
            let part = match Part::from(&b[
                (offset+(i*part_size) as usize)..
                (offset+((i+1)*part_size) as usize)
            ], part_content_num, part_content_list_size) {
                // we have to manually check this error because there is an error this throws that we want to handle.
                Ok(a) => a,
                Err(err) => {
                    if err.to_string() == "out of bounds" {
                        // for some unknown reason, sometimes the part size is not correct.
                        // this should be investigated. but for now, if it's ever problem, just load the rest of the block in.
                        Part::from(&b[(offset+(i*part_size) as usize)..], part_content_num, part_content_list_size)?
                    } else {
                        return Err(err.into());
                    }
                }
            };
            parts.push(part);
            //offset += part_size as usize;
            offset += part_size as usize;
            // offset is now at the "optional OSA script data".
            // skip this.
            part_size = byte::u16_from_u8(&b[offset..offset + 0x02]);
            //println!("osa size: {}",part_size);
            offset += (part_size/8) as usize;
            //offset += new_offset;
        };

        // name and stack script, both terminated by nil
        let mut stack: Vec<char> = Vec::new();
        loop {
            let ch = (&b)[offset];
            if ch == 0 {
                break;
            }
            stack.push(macroman_to_char(ch)); // blindly unwrap because we can be certain it's a valid mac roman character
            offset += 1;
        }
        let name: String = (&stack).iter().collect();
        let mut stack: Vec<char> = Vec::new();
        loop {
            let ch = (&b)[offset];
            if ch == 0 {
                break;
            }
            stack.push(macroman_to_char(ch)); // blindly unwrap because we can be certain it's a valid mac roman character
            offset += 1;
        }
        let code: String = (&stack).iter().collect();
        Ok(Card{
            id,
            bitmap_block_id,
            flags,
            parts,
            name,
            script: code,
        })
    }
}