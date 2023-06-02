use ascii_converter::decimals_to_string;
use eyre::eyre;
use std::{error::Error, ffi};

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
    masklength_redundant: ffi::c_int,
    mask: &'a ffi::CStr,
}

/// WOBA-formatted picture
impl<'a> picture<'a> {
    /// The width
    pub fn width(&self) -> usize {
        *&self.width as usize
    }
    /// The height
    pub fn height(&self) -> usize {
        *&self.height as usize
    }
    /// The depth
    pub fn depth(&self) -> usize {
        *&self.depth as usize
    }
    /// The greyscale-mask
    pub fn greyscalemask(&self) -> bool {
        *&self.greyscalemask == 1
    }
    // The raw bitmap of the
    pub fn bitmap_raw(&self) -> &[u8] {
        &self.bitmap.to_bytes()[..self.bitmaplength as usize - 1]
    }
    pub fn blank_pbm(&self) -> Vec<u8> {
        format!("P4 {} {} ", &self.width(), &self.height())
            .as_bytes()
            .to_vec()
    }
    pub fn bitmap_pbm(&self) -> Vec<u8> {
        let j = format!("P4 {} {} ", &self.width(), &self.height());
        [j.as_bytes(), (&self).bitmap_raw()].concat()
    }
    pub fn mask_raw(&self) -> Option<&[u8]> {
        // redundany check: check if the struct above read the correct mask length by seeing if it read it twice
        if self.masklength != self.masklength_redundant {
            // this means the program is reading invalid memory, which indicates that there is no mask.
            None
        } else {
            Some(&self.mask.to_bytes()[..self.masklength as usize - 1])
        }
    }
    pub fn mask_pbm(&self) -> Option<Vec<u8>> {
        match (&self).mask_raw() {
            Some(a) => {
                let j = format!("P4 {} {} ", &self.width(), &self.height());
                Some([j.as_bytes(), a].concat())
            }
            None => None,
        }
    }
}

#[link(name = "stackimport")]
extern "C" {
    fn new_picture_with_params<'a>(
        w: ffi::c_int,
        h: ffi::c_int,
        d: ffi::c_int,
        greymask: ffi::c_int,
    ) -> picture<'a>;
    fn woba_decode(p: &picture, woba: &ffi::CStr);
    fn get_error_str<'a>() -> &'a ffi::CStr;
}

pub fn decode(b: &[u8]) -> Result<picture, eyre::ErrReport> {
    let p: picture;
    unsafe {
        p = new_picture_with_params(0, 0, 0, 0);
        woba_decode(&p, &ffi::CStr::from_bytes_with_nul(b)?);

        let err = get_error_str();
        if err.to_bytes().len() > 1 && err.to_bytes().len() < i32::MAX as usize {
            return Err(eyre!(err.to_str()?));
        }
    };
    Ok(p)
}
