use std::{ffi, error::Error};
use eyre::eyre;

#[derive(Debug)]
#[repr(C)]
pub struct picture<'a> {
    pub width: ffi::c_int,
    pub height: ffi::c_int,
    pub depth: ffi::c_int,
    pub greyscalemask: ffi::c_int,

    pub rowlength: ffi::c_int,
    pub maskrowlength: ffi::c_int,
    pub bitmaplength: ffi::c_int,
    pub bitmap: &'a ffi::CStr,
    pub masklength: ffi::c_int,
    pub mask: &'a ffi::CStr,
}

#[link(name = "stackimport")]
extern "C" {
    fn new_picture_with_params<'a>(w: ffi::c_int, h: ffi::c_int, d: ffi::c_int, greymask: ffi::c_int) -> picture<'a>;
    fn woba_decode(p: &picture, woba: &ffi::CStr);
    fn get_error_str<'a>() -> &'a ffi::CStr;
}

pub fn decode(b: &[u8], width: u16, height: u16, depth: i32) -> Result<picture, eyre::ErrReport> {
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