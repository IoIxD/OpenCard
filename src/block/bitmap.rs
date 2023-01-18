use image::GrayImage;

use super::data_layout::BitmapLayout as bi;
use crate::{byte};
use stackimport::decode;

pub struct Bitmap {
    card_top: u32,
    card_left: u32,
    card_right: u32,
    card_bottom: u32,
    mask_top: u32,
    mask_left: u32,
    mask_bottom: u32,
    mask_right: u32,
    image_top: u32,
    image_left: u32,
    image_bottom: u32,
    image_right: u32,

    pub mask: Option<GrayImage>,
    pub image: Option<GrayImage>,
}

impl Bitmap {
    pub fn from(b: &[u8]) -> Self {
        let (
            card_top, card_left, card_bottom, card_right
        ) = (
            byte::u16_from_u8(&b[bi::CardTopStart()..bi::CardTopEnd()]) as u32,
            byte::u16_from_u8(&b[bi::CardLeftStart()..bi::CardLeftEnd()]) as u32,
            byte::u16_from_u8(&b[bi::CardBottomStart()..bi::CardBottomEnd()]) as u32,
            byte::u16_from_u8(&b[bi::CardRightStart()..bi::CardRightEnd()]) as u32,
        );
        let (
            mask_top, mask_left, mask_bottom, mask_right
        ) = (
            byte::u16_from_u8(&b[bi::MaskTopStart()..bi::MaskTopEnd()]) as u32,
            byte::u16_from_u8(&b[bi::MaskLeftStart()..bi::MaskLeftEnd()]) as u32,
            byte::u16_from_u8(&b[bi::MaskBottomStart()..bi::MaskBottomEnd()]) as u32,
            byte::u16_from_u8(&b[bi::MaskRightStart()..bi::MaskRightEnd()]) as u32,
        );
        let (
            image_top, image_left, image_bottom, image_right
        ) = (
            byte::u16_from_u8(&b[bi::ImageTopStart()..bi::ImageTopEnd()]) as u32,
            byte::u16_from_u8(&b[bi::ImageLeftStart()..bi::ImageLeftEnd()]) as u32,
            byte::u16_from_u8(&b[bi::ImageBottomStart()..bi::ImageBottomEnd()]) as u32,
            byte::u16_from_u8(&b[bi::ImageRightStart()..bi::ImageRightEnd()]) as u32,
        );

        let mask_size = byte::u32_from_u8(&b[bi::MaskDataSizeStart()..bi::MaskDataSizeEnd()]) as usize;
        let image_size = byte::u32_from_u8(&b[bi::ImageDataSizeStart()..bi::ImageDataSizeEnd()]) as usize;

        let width = byte::u16_from_u8(&b[bi::ImageRightStart()..bi::ImageRightEnd()]);
        let height = byte::u16_from_u8(&b[bi::ImageBottomStart()..bi::ImageBottomEnd()]);
        let pic_chunk = &b[0x08..];
        let pic = decode(&pic_chunk,width,height,8);

        let p = pic.unwrap();
        println!("{}",p.width);
        println!("{}",p.height);
        let b = p.bitmap.to_bytes();
        for i in b {
            print!("{}",i);
        }

        Bitmap{
            card_top,
            card_left,
            card_right,
            card_bottom,
            mask_top,
            mask_left,
            mask_bottom,
            mask_right,
            image_top,
            image_left,
            image_bottom,
            image_right,
            mask: None,
            image: None,
        }
    }
}