use std::error::Error;
use std::io::Cursor;

use image::{GrayImage};
use image::io::Reader as ImageReader;

use super::data_layout::BitmapLayout as bi;
use stackimport::decode;

#[derive(Debug,Clone)]
pub struct Bitmap {
    pub image: GrayImage,
}

impl Bitmap {
    pub fn from(b: &[u8]) -> Result<Self, Box<dyn Error>> {
        let pic_chunk = &b[bi::Filler0Start()..];
        let pic = decode(&pic_chunk);

        let p = pic.unwrap();
        let bpbm: Vec<u8> = p.bitmap_pbm();
        let mpbm: Option<Vec<u8>> = p.mask_pbm();
        let blpbm: Vec<u8> = p.blank_pbm();
        let image = match mpbm {
            Some(a) => {
                ImageReader::new(Cursor::new([bpbm,a].concat())).with_guessed_format()?.decode()?
            }
            None => {
                ImageReader::new(Cursor::new([bpbm,blpbm].concat())).with_guessed_format()?.decode()?
            }
        };
        Ok(Bitmap{
            image: image.as_luma8().unwrap().clone()
        })
    }
}