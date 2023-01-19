use std::{ffi, error::Error};
use eyre::eyre;
use ascii_converter::decimals_to_string;

#[derive(Debug)]
#[repr(C)]
pub struct picture<'a> {
    width: ffi::c_int,
    height: ffi::c_int,
    depth: ffi::c_int,
    greyscalemask: ffi::c_int,

    rowlength: ffi::c_int,
    maskrowlength: ffi::c_int,
    bitmaplength: ffi::c_int,
    bitmap: &'a ffi::CStr,
    masklength: ffi::c_int,
    mask: &'a ffi::CStr,
}

impl<'a> picture<'a> {
    pub fn width(&self) -> usize {
        *&self.width as usize
    }
    pub fn height(&self) -> usize {
        *&self.height as usize
    }
    pub fn depth(&self) -> usize {
        *&self.depth as usize
    }
    pub fn greyscalemask(&self) -> bool {
        *&self.greyscalemask == 1
    }
    pub fn bitmap_raw(&self) -> &[u8] {
        &self.bitmap.to_bytes()
    }
    pub fn blank_pbm(&self) -> Vec<u8> {
        format!("P4 {} {} ",&self.width(),&self.height()).as_bytes().to_vec()
    }
    pub fn bitmap_pbm(&self) -> Vec<u8> {
        let j = format!("P4 {} {} ",&self.width(),&self.height());
        [j.as_bytes(), (&self).bitmap_raw()].concat()
    }
    pub fn mask_raw(&self) -> Option<&[u8]> {
        if self.masklength <= 0 || self.masklength >= i16::MAX as i32 {
            None
        } else {
            Some(&self.mask.to_bytes())
        }
    }
    pub fn mask_pbm(&self) -> Option<Vec<u8>> {
        if self.masklength <= 0 || self.masklength >= i16::MAX as i32 {
            None
        } else {
            let j = format!("P4 {} {} ",&self.width(),&self.height());
            Some([j.as_bytes(), (&self).mask_raw().unwrap()].concat())
        }
    }
}

#[link(name = "stackimport")]
extern "C" {
    fn new_picture_with_params<'a>(w: ffi::c_int, h: ffi::c_int, d: ffi::c_int, greymask: ffi::c_int) -> picture<'a>;
    fn woba_decode(p: &picture, woba: &ffi::CStr);
    fn get_error_str<'a>() -> &'a ffi::CStr;
}

pub fn decode(b: &[u8]) -> Result<picture, eyre::ErrReport> {
    let p: picture;
    unsafe {
        p = new_picture_with_params(0,0,0,0);
        woba_decode(&p,&ffi::CStr::from_bytes_with_nul_unchecked(b));

        let err = get_error_str();
        if err.to_bytes().len() > 1 && err.to_bytes().len() < i32::MAX as usize {
            return Err(eyre!(err.to_str()?));
        }
    };
    Ok(p)
}