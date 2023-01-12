use image::GrayImage;

pub struct Bitmap<'a> {
    card_top: u16,
    card_left: u16,
    card_right: u16,
    card_bottom: u16,
    mask_top: u16,
    mask_left: u16,
    mask_bottom: u16,
    mask_right: u16,
    image_top: u16,
    image_left: u16,
    image_bottom: u16,
    image_right: u16,

    mask: &'a GrayImage,
    image: &'a GrayImage,
}
