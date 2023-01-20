use super::bitmap::Bitmap;
use super::card::Card;
use super::part::Part;

pub struct Background<'a> {
    bitmap: &'a Bitmap,
    flags: u16,

    card_ids: Vec<u16>,
    next: Option<&'a Background<'a>>,
    prev: Option<&'a Background<'a>>,

    name: &'a str,
    script: &'a str,
}