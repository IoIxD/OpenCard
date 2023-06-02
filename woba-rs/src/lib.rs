use eyre::eyre;
use image::io::Reader as ImageReader;
use image::GrayImage;
use std::error::Error;
use std::ffi;
use std::io::Cursor;

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
    pub fn as_image(self) -> Result<GrayImage, Box<dyn Error>> {
        let bpbm = self.bitmap_pbm();
        let mpbm = self.mask_pbm();
        let blpbm = self.blank_pbm();
        let image = match mpbm {
            Some(a) => ImageReader::new(Cursor::new([bpbm, a].concat()))
                .with_guessed_format()?
                .decode()?,
            None => ImageReader::new(Cursor::new([bpbm, blpbm].concat()))
                .with_guessed_format()?
                .decode()?,
        };
        Ok(image.as_luma8().unwrap().clone())
    }

    fn bitmap_raw(&self) -> &[u8] {
        &self.bitmap.to_bytes()[..self.bitmaplength as usize - 1]
    }

    fn blank_pbm(&self) -> Vec<u8> {
        // ...why is this a thing again?
        format!("P4 {} {} ", &self.width(), &self.height())
            .as_bytes()
            .to_vec()
    }

    fn bitmap_pbm(&self) -> Vec<u8> {
        let j = format!("P4 {} {} ", &self.width(), &self.height());
        [j.as_bytes(), (&self).bitmap_raw()].concat()
    }
    fn mask_raw(&self) -> Option<&[u8]> {
        // redundany check: check if the struct above read the correct mask length by seeing if it read it twice
        if self.masklength != self.masklength_redundant {
            // this means the program is reading invalid memory, which indicates that there is no mask.
            None
        } else {
            Some(&self.mask.to_bytes()[..self.masklength as usize - 1])
        }
    }

    fn mask_pbm(&self) -> Option<Vec<u8>> {
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
        woba_decode(&p, &ffi::CStr::from_bytes_with_nul_unchecked(b));

        let err = get_error_str();
        if err.to_bytes().len() > 1 && err.to_bytes().len() < i32::MAX as usize {
            return Err(eyre!(err.to_str()?));
        }
    };
    Ok(p)
}
