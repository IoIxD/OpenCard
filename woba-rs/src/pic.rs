use std::{
    error::Error,
    ffi::{self, c_char},
    io::Cursor,
};

use image::{io::Reader as ImageReader, GrayImage};

use crate::cbuf::CBuf;

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
    pub fn width(&mut self) -> usize {
        *&mut self.width as usize
    }
    /// The height
    pub fn height(&mut self) -> usize {
        *&mut self.height as usize
    }
    /// The depth
    pub fn depth(&mut self) -> usize {
        *&mut self.depth as usize
    }
    /// The greyscale-mask
    pub fn greyscalemask(&mut self) -> bool {
        *&mut self.greyscalemask == 1
    }
    pub fn as_image(&mut self) -> Result<GrayImage, Box<dyn Error>> {
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

    fn bitmap_raw(&mut self) -> &[u8] {
        &mut self.bitmap.to_bytes()[..self.bitmaplength as usize - 1]
    }

    fn blank_pbm(&mut self) -> Vec<u8> {
        // ...why is this a thing again?
        format!("P4 {} {} ", &mut self.width(), &mut self.height())
            .as_bytes()
            .to_vec()
    }

    fn bitmap_pbm(&mut self) -> Vec<u8> {
        let j = format!("P4 {} {} ", &mut self.width(), &mut self.height());
        [j.as_bytes(), self.bitmap_raw()].concat()
    }
    fn mask_raw(&mut self) -> Option<&[u8]> {
        // redundany check: check if the struct above read the correct mask length by seeing if it read it twice
        if self.masklength != self.masklength_redundant {
            // this means the program is reading invalid memory, which indicates that there is no mask.
            None
        } else {
            Some(&mut self.mask.to_bytes()[..self.masklength as usize - 1])
        }
    }

    fn mask_pbm(&mut self) -> Option<Vec<u8>> {
        match self.mask_raw() {
            Some(a) => {
                let j = format!("P4 {} {} ", &mut self.width(), &mut self.height());
                Some([j.as_bytes(), a].concat())
            }
            None => None,
        }
    }

    pub unsafe fn reinit(&mut self, w: u16, h: u16, d: u16, greymask: bool) -> u16 {
        picture_reinit(
            self as *mut Self,
            w.into(),
            h.into(),
            d.into(),
            greymask as u8 as i32,
        )
    }

    pub unsafe fn gwidth(&mut self) -> u16 {
        picture_gwidth(self as *mut Self)
    }
    pub unsafe fn gheight(&mut self) -> u16 {
        picture_gheight(self as *mut Self)
    }
    pub unsafe fn gdepth(&mut self) -> u16 {
        picture_gdepth(self as *mut Self)
    }
    pub unsafe fn gmaskdepth(&mut self) -> u16 {
        picture_gmaskdepth(self as *mut Self)
    }
    pub unsafe fn bitmapsize(&mut self) -> u16 {
        picture_bitmapsize(self as *mut Self)
    }
    pub unsafe fn masksize(&mut self) -> u16 {
        picture_masksize(self as *mut Self)
    }

    pub unsafe fn coordbyteoffset(&mut self, a: u16, b: u16) -> u16 {
        picture_coordbyteoffset(self as *mut Self, a.into(), b.into())
    }
    pub unsafe fn coordbitmask(&mut self, a: u16, b: u16) -> u32 {
        picture_coordbitmask(self as *mut Self, a.into(), b.into())
    }
    pub unsafe fn maskcoordbyteoffset(&mut self, a: u16, b: u16) -> u16 {
        picture_maskcoordbyteoffset(self as *mut Self, a.into(), b.into())
    }
    pub unsafe fn maskcoordbitmask(&mut self, a: u16, b: u16) -> u32 {
        picture_maskcoordbitmask(self as *mut Self, a.into(), b.into())
    }

    pub unsafe fn memcopyin<A>(&mut self, a: &mut [i8], b: u16, c: u16, d: Option<A>) -> u16
    where
        A: Into<u16>,
    {
        match d {
            Some(d) => picture_memcopyin_b(
                self as *mut Self,
                a.as_mut_ptr(),
                b.into(),
                c.into(),
                d.into().into(),
            ),
            None => picture_memcopyin(self as *mut Self, a.as_mut_ptr(), b.into(), c.into()),
        }
    }
    pub unsafe fn maskmemcopyin<A>(&mut self, a: &mut [i8], b: u16, c: u16, d: Option<A>) -> u16
    where
        A: Into<u16>,
    {
        match d {
            Some(d) => picture_maskmemcopyin_b(
                self as *mut Self,
                a.as_mut_ptr(),
                b.into(),
                c.into(),
                d.into().into(),
            ),
            None => picture_maskmemcopyin(self as *mut Self, a.as_mut_ptr(), b.into(), c.into()),
        }
    }

    pub unsafe fn memcopyout_from_start_buf(
        &mut self,
        dest: &mut CBuf,
        start: u16,
        count: u16,
    ) -> u16 {
        picture_memcopyout_from_start_buf(self as *mut Self, dest, start.into(), count.into())
    }
    pub unsafe fn memcopyout_x_y_buf(
        &mut self,
        dest: &mut CBuf,
        x: u16,
        y: u16,
        count: u16,
    ) -> u16 {
        picture_memcopyout_x_y_buf(self as *mut Self, dest, x.into(), y.into(), count.into())
    }
    pub unsafe fn memcopyout_from_start_char(
        &mut self,
        dest: &mut [i8],
        start: u16,
        count: u16,
    ) -> u16 {
        picture_memcopyout_from_start_char(
            self as *mut Self,
            dest.as_mut_ptr(),
            start.into(),
            count.into(),
        )
    }
    pub unsafe fn memcopyout_x_y_char(
        &mut self,
        dest: &mut [i8],
        x: u16,
        y: u16,
        count: u16,
    ) -> u16 {
        picture_memcopyout_x_y_char(
            self as *mut Self,
            dest.as_mut_ptr(),
            x.into(),
            y.into(),
            count.into(),
        )
    }
    pub unsafe fn maskmemcopyout_from_start_char(
        &mut self,
        dest: &mut [i8],
        start: u16,
        count: u16,
    ) -> u16 {
        picture_maskmemcopyout_from_start_char(
            self as *mut Self,
            dest.as_mut_ptr(),
            start.into(),
            count.into(),
        )
    }
    pub unsafe fn maskmemcopyout_x_y_char(
        &mut self,
        dest: &mut [i8],
        x: u16,
        y: u16,
        count: u16,
    ) -> u16 {
        picture_maskmemcopyout_x_y_char(
            self as *mut Self,
            dest.as_mut_ptr(),
            x.into(),
            y.into(),
            count.into(),
        )
    }
    pub unsafe fn maskmemcopyout_x_y_buf(
        &mut self,
        dest: &mut CBuf,
        x: u16,
        y: u16,
        count: u16,
    ) -> u16 {
        picture_maskmemcopyout_x_y_buf(
            self as *mut Self,
            dest as *mut CBuf,
            x.into(),
            y.into(),
            count.into(),
        )
    }

    pub unsafe fn memfill<A>(&mut self, a: ffi::c_char, b: u16, c: u16, d: Option<A>) -> u16
    where
        A: Into<u16>,
    {
        match d {
            Some(d) => picture_memfill_b(
                self as *mut Self,
                a.into(),
                b.into(),
                c.into(),
                d.into().into(),
            ),
            None => picture_memfill(self as *mut Self, a.into(), b.into(), c.into()),
        }
    }
    pub unsafe fn maskmemfill<A>(&mut self, a: ffi::c_char, b: u16, c: u16, d: Option<A>) -> u16
    where
        A: Into<u16>,
    {
        match d {
            Some(d) => picture_maskmemfill_b(
                self as *mut Self,
                a.into(),
                b.into(),
                c.into(),
                d.into().into(),
            ),
            None => picture_maskmemfill(self as *mut Self, a.into(), b.into(), c.into()),
        }
    }
    pub unsafe fn buildmaskfromsurroundings(&mut self) {
        picture_buildmaskfromsurroundings(self as *mut Self);
    }
    pub unsafe fn scanstartingatpixel(&mut self, x: u16, y: u16) -> u16 {
        picture_scanstartingatpixel(self as *mut Self, x.into(), y.into())
    }
    pub unsafe fn debugprint(&mut self) {
        picture_debugprint(self as *mut Self);
    }

    pub unsafe fn copyrow(&mut self, a: u16, b: u16) -> u16 {
        picture_copyrow(self as *mut Self, a.into(), b.into())
    }
    pub unsafe fn maskcopyrow(&mut self, a: u16, b: u16) -> u16 {
        picture_maskcopyrow(self as *mut Self, a.into(), b.into())
    }

    pub unsafe fn fixcolor(&mut self, a: ffi::c_uint) -> u32 {
        picture_fixcolor(self as *mut Self, a.into())
    }
    pub unsafe fn dupcolor(&mut self, a: ffi::c_uint) -> u32 {
        picture_dupcolor(self as *mut Self, a.into())
    }

    pub unsafe fn getpixel(&mut self, a: u16, b: u16) -> u32 {
        picture_getpixel(self as *mut Self, a.into(), b.into())
    }
    pub unsafe fn setpixel(&mut self, a: u16, b: u16, c: u16) -> u16 {
        picture_setpixel(self as *mut Self, a.into(), b.into(), c.into())
    }
    pub unsafe fn maskgetpixel(&mut self, a: u16, b: u16) -> u32 {
        picture_maskgetpixel(self as *mut Self, a.into(), b.into())
    }
    pub unsafe fn masksetpixel(&mut self, a: u16, b: u16, c: u16) -> u16 {
        picture_masksetpixel(self as *mut Self, a.into(), b.into(), c.into())
    }

    /*
    fn picture___directcopybmptomask(p: *mut picture);

    fn picture_writefile(p: *mut picture, a: &mut [i8]);
    fn picture_writebitmapandmasktopbm(p: *mut picture, f: &mut [i8]);
    fn picture_writebitmaptopbm(p: *mut picture, f: &mut [i8]);
    fn picture_writemasktopbm(p: *mut picture, f: &mut [i8]);

    fn picture_readfile(p: *mut picture, a: &mut [i8]); */

    pub unsafe fn __directcopybmptomask(&mut self) {
        picture___directcopybmptomask(self as *mut Self)
    }

    pub unsafe fn writefile(&mut self, a: &mut [i8]) {
        picture_writefile(self as *mut Self, a.as_mut_ptr())
    }

    pub unsafe fn writebitmapandmasktopbm(&mut self, a: &mut [i8]) {
        picture_writebitmapandmasktopbm(self as *mut Self, a.as_mut_ptr())
    }

    pub unsafe fn writebitmaptopbm(&mut self, a: &mut [i8]) {
        picture_writebitmaptopbm(self as *mut Self, a.as_mut_ptr())
    }

    pub unsafe fn writemasktopbm(&mut self, a: &mut [i8]) {
        picture_writemasktopbm(self as *mut Self, a.as_mut_ptr())
    }

    pub unsafe fn readfile(&mut self, a: &mut [i8]) {
        picture_readfile(self as *mut Self, a.as_mut_ptr());
    }
}

#[link(name = "stackimport")]
extern "C" {
    pub fn new_picture_with_params<'a>(
        w: ffi::c_int,
        h: ffi::c_int,
        d: ffi::c_int,
        greymask: ffi::c_int,
    ) -> picture<'a>;
    pub fn woba_decode(p: &picture, woba: &ffi::CStr);
    pub fn get_error_str<'a>() -> &'a ffi::CStr;

    fn picture_new<'a>() -> picture<'a>;
    fn picture_drop(p: *mut picture);
    fn picture_reinit(
        p: *mut picture,
        w: ffi::c_int,
        h: ffi::c_int,
        d: ffi::c_int,
        greymask: ffi::c_int,
    ) -> u16;

    fn picture_gwidth(p: *mut picture) -> u16;
    fn picture_gheight(p: *mut picture) -> u16;
    fn picture_gdepth(p: *mut picture) -> u16;
    fn picture_gmaskdepth(p: *mut picture) -> u16;
    fn picture_bitmapsize(p: *mut picture) -> u16;
    fn picture_masksize(p: *mut picture) -> u16;

    fn picture_coordbyteoffset(p: *mut picture, a: ffi::c_int, b: ffi::c_int) -> u16;
    fn picture_coordbitmask(p: *mut picture, a: ffi::c_int, b: ffi::c_int) -> u32;
    fn picture_maskcoordbyteoffset(p: *mut picture, a: ffi::c_int, b: ffi::c_int) -> u16;
    fn picture_maskcoordbitmask(p: *mut picture, a: ffi::c_int, b: ffi::c_int) -> u32;

    fn picture_memcopyin(p: *mut picture, a: *mut c_char, b: ffi::c_int, c: ffi::c_int) -> u16;
    fn picture_memcopyin_b(
        p: *mut picture,
        a: *mut c_char,
        b: ffi::c_int,
        c: ffi::c_int,
        d: ffi::c_int,
    ) -> u16;
    fn picture_maskmemcopyin(p: *mut picture, a: *mut c_char, b: ffi::c_int, c: ffi::c_int) -> u16;
    fn picture_maskmemcopyin_b(
        p: *mut picture,
        a: *mut c_char,
        b: ffi::c_int,
        c: ffi::c_int,
        d: ffi::c_int,
    ) -> u16;

    fn picture_memcopyout_from_start_char(
        p: *mut picture,
        dest: *mut c_char,
        start: ffi::c_int,
        count: ffi::c_int,
    ) -> u16;
    fn picture_memcopyout_x_y_char(
        p: *mut picture,
        dest: *mut c_char,
        x: ffi::c_int,
        y: ffi::c_int,
        count: ffi::c_int,
    ) -> u16;
    fn picture_maskmemcopyout_from_start_char(
        p: *mut picture,
        dest: *mut c_char,
        start: ffi::c_int,
        count: ffi::c_int,
    ) -> u16;
    fn picture_maskmemcopyout_x_y_char(
        p: *mut picture,
        dest: *mut c_char,
        x: ffi::c_int,
        y: ffi::c_int,
        count: ffi::c_int,
    ) -> u16;

    fn picture_memcopyout_from_start_buf(
        p: *mut picture,
        dest: *mut CBuf,
        start: ffi::c_int,
        count: ffi::c_int,
    ) -> u16;
    fn picture_memcopyout_x_y_buf(
        p: *mut picture,
        dest: *mut CBuf,
        x: ffi::c_int,
        y: ffi::c_int,
        count: ffi::c_int,
    ) -> u16;
    fn picture_maskmemcopyout_from_start(
        p: *mut picture,
        dest: *mut CBuf,
        start: ffi::c_int,
        count: ffi::c_int,
    ) -> u16;
    fn picture_maskmemcopyout_x_y_buf(
        p: *mut picture,
        dest: *mut CBuf,
        x: ffi::c_int,
        y: ffi::c_int,
        count: ffi::c_int,
    ) -> u16;

    fn picture_memfill(p: *mut picture, a: ffi::c_char, b: ffi::c_int, c: ffi::c_int) -> u16;
    fn picture_memfill_b(
        p: *mut picture,
        a: ffi::c_char,
        b: ffi::c_int,
        c: ffi::c_int,
        d: ffi::c_int,
    ) -> u16;
    fn picture_maskmemfill(p: *mut picture, a: ffi::c_char, b: ffi::c_int, c: ffi::c_int) -> u16;
    fn picture_maskmemfill_b(
        p: *mut picture,
        a: ffi::c_char,
        b: ffi::c_int,
        c: ffi::c_int,
        d: ffi::c_int,
    ) -> u16;

    fn picture_buildmaskfromsurroundings(p: *mut picture);
    fn picture_scanstartingatpixel(p: *mut picture, x: ffi::c_int, y: ffi::c_int) -> u16;
    fn picture_debugprint(p: *mut picture);

    fn picture_copyrow(p: *mut picture, a: ffi::c_int, b: ffi::c_int) -> u16;
    fn picture_maskcopyrow(p: *mut picture, a: ffi::c_int, b: ffi::c_int) -> u16;

    fn picture_fixcolor(p: *mut picture, a: ffi::c_uint) -> u32;
    fn picture_dupcolor(p: *mut picture, a: ffi::c_uint) -> u32;

    fn picture_getpixel(p: *mut picture, a: ffi::c_int, b: ffi::c_int) -> u32;
    fn picture_setpixel(p: *mut picture, a: ffi::c_int, b: ffi::c_int, c: ffi::c_int) -> u16;
    fn picture_maskgetpixel(p: *mut picture, a: ffi::c_int, b: ffi::c_int) -> u32;
    fn picture_masksetpixel(p: *mut picture, a: ffi::c_int, b: ffi::c_int, c: ffi::c_int) -> u16;

    fn picture___directcopybmptomask(p: *mut picture);

    fn picture_writefile(p: *mut picture, a: *mut c_char);
    fn picture_writebitmapandmasktopbm(p: *mut picture, f: *mut c_char);
    fn picture_writebitmaptopbm(p: *mut picture, f: *mut c_char);
    fn picture_writemasktopbm(p: *mut picture, f: *mut c_char);

    fn picture_readfile(p: *mut picture, a: *mut c_char);
}
