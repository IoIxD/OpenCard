use super::bitmap::Bitmap;
use super::card::Card;
use super::part::Part;

pub struct Background<'a> {
    bitmap: &'a Bitmap<'a>,
    flags: u16,

    cards: &'a [&'a Card<'a>],
    next: Option<&'a Background<'a>>,
    prev: Option<&'a Background<'a>>,

    parts: &'a [&'a Part<'a>],

    name: &'a str,
    script: &'a str,
}