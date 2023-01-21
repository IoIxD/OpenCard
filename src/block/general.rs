use std::collections::HashMap;

use super::{bitmap::Bitmap, font::Font, background::Background, style::Style, card::Card, part::Part};

#[derive(Debug,Clone)]
pub enum Block {
    Background(Background),
    Bitmap(Bitmap),
    Card(Card),
    Font(Font),
    Part(Part),
    Style(HashMap<u32, Style>)
}

impl Block {
    pub fn is_background(&self) -> bool {
        matches!(&self, Block::Background(_))
    }
    pub fn is_bitmap(&self) -> bool {
        matches!(&self, Block::Bitmap(_))
    }
    pub fn is_card(&self) -> bool {
        matches!(&self, Block::Card(_))
    }
    pub fn is_font(&self) -> bool {
        matches!(&self, Block::Font(_))
    }
    pub fn is_part(&self) -> bool {
        matches!(&self, Block::Part(_))
    }
    pub fn is_style(&self) -> bool {
        matches!(&self, Block::Style(_))
    }
    pub fn get_background<'a>(&self) -> Background {
        if let Block::Background(a) = &self {
            a.clone()
        } else {
            panic!("not a background");
        }
    }
    pub fn get_bitmap<'a>(&self) -> Bitmap {
        if let Block::Bitmap(a) = &self {
            a.clone()
        } else {
            panic!("not a bitmap");
        }
    }
    pub fn get_card<'a>(&self) -> Card {
        if let Block::Card(a) = &self {
            a.clone()
        } else {
            panic!("not a card");
        }
    }
    pub fn get_font<'a>(&self) -> Font {
        if let Block::Font(a) = &self {
            a.clone()
        } else {
            panic!("not a font");
        }
    }
    pub fn get_part<'a>(&self) -> Part {
        if let Block::Part(a) = &self {
            a.clone()
        } else {
            panic!("not a part");
        }
    }
    pub fn get_style<'a>(&self) -> HashMap<u32, Style> {
        if let Block::Style(a) = &self {
            a.clone()
        } else {
            panic!("not a style");
        }
    }
}