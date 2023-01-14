use super::background::Background;
use super::part::Part;

pub struct Card<'a> {
    id: u32,
    bitmap_block_id: u32,
    flags: u32,

    background: &'a Background<'a>,
    parts: &'a [&'a Part<'a>],

    name: &'a str,
    script: &'a str,
}
