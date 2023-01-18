use super::{bitmap::Bitmap, font::Font, background::Background, style::Style, card::Card, part::Part};

pub enum Block<'a> {
    Background(Background<'a>),
    Bitmap(Bitmap),
    Card(Card<'a>),
    Font(Font<'a>),
    Part(Part<'a>),
    Style(Style<'a>)
}