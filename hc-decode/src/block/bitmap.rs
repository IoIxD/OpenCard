use eyre::{eyre, ErrReport};
use image::GrayImage;

use std::error::Error;

use super::data_layout::BitmapLayout as bi;
use woba::decode;

#[derive(Debug, Clone)]
pub struct Bitmap {
    pub image: GrayImage,
}

impl Bitmap {
    pub fn from(b: &[u8]) -> Result<Self, Box<dyn Error>> {
        let pic_chunk = &b[bi::Filler0Start()..];
        let pic = decode(&pic_chunk)?;
        Ok(Bitmap {
            image: pic.as_image()?,
        })
    }
}
