use eyre::{eyre, ErrReport};

use std::error::Error;

use super::data_layout::CardLayout as c;
use super::part::Part;
use crate::byte::{self, byte_range};
use crate::macroman::macroman_to_char;

#[derive(Debug, Clone)]
pub struct Card {
    pub id: u32,
    pub bitmap_block_id: u32,
    pub flags: u16,
    pub parts: Vec<Part>,

    pub name: String,
    pub script: String,
}

impl Card {
    pub fn from(b: &[u8]) -> Result<Self, Box<dyn Error>> {
        /*for i in 0..b.len() {
            println!("{:#09x}\t\t{}",i,b[i]);
        }*/
        let id = byte_range!(u32, b, c::BlockID);
        let bitmap_block_id = byte_range!(u32, b, c::BitmapID);
        let flags = byte_range!(u16, b, c::Flags);
        let part_num = byte_range!(u16, b, c::PartNum);
        let part_content_num = byte_range!(u16, b, c::PartContentNum);
        let part_content_list_size = byte_range!(u32, b, c::PartContentListSize);

        let mut offset = c::PartContentListSizeEnd();
        let mut parts: Vec<Part> = Vec::new();
        #[allow(unused_assignments)]
        let mut part_size = 0;
        for i in 0..part_num {
            // i don't know
            if offset + 0x02 >= parts.len() {
                continue;
            }
            part_size = byte::u16_from_u8(&b[offset..offset + 0x02]);
            let part = match Part::from(
                &b[(offset + (i * part_size) as usize)..(offset + ((i + 1) * part_size) as usize)],
                part_content_num,
                part_content_list_size,
            ) {
                // we have to manually check this error because there is an error this throws that we want to handle.
                Ok(a) => a,
                Err(err) => {
                    if err.to_string() == "out of bounds" {
                        // for some unknown reason, sometimes the part size is not correct.
                        // this should be investigated. but for now, if it's ever problem, just load the rest of the block in.
                        Part::from(
                            &b[(offset + (i * part_size) as usize)..],
                            part_content_num,
                            part_content_list_size,
                        )?
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
            offset += (part_size / 8) as usize;
        }

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
        Ok(Card {
            id,
            bitmap_block_id,
            flags,
            parts,
            name,
            script: code,
        })
    }
}
