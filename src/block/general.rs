use super::{bitmap::Bitmap, font::Font, list::List, page::Page, background::Background, style::Style, card::Card, part::Part};

pub enum Block<'a> {
    Background(Background<'a>),
    Bitmap(Bitmap<'a>),
    Card(Card<'a>),
    Font(Font<'a>),
    List(List<'a>),
    Page(Page<'a>),
    Part(Part<'a>),
    Style(Style<'a>)
}