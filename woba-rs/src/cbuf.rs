use std::{
    ffi::{c_char, c_int, c_short, c_ushort, CStr},
    ops::{Index, IndexMut},
    u16,
};

pub struct CBuf;
impl CBuf {
    pub unsafe fn new() -> CBuf {
        cbuf_new_with_size(0)
    }
    pub unsafe fn new_with_size(inSize: usize) -> CBuf {
        cbuf_new_with_size(inSize)
    }
    pub unsafe fn new_with_uhh(startOffs: usize, amount: usize) -> CBuf {
        cbuf_new_with_uhh(startOffs, amount)
    }
    pub unsafe fn resize(&mut self, inSize: usize) {
        cbuf_resize(self as *mut Self, inSize)
    }
    pub unsafe fn memcpy_char(
        &mut self,
        toOffs: usize,
        fromPtr: *const c_char,
        fromOffs: usize,
        amount: usize,
    ) {
        cbuf_memcpy_char(self as *mut Self, toOffs, fromPtr, fromOffs, amount)
    }
    pub unsafe fn memcpy_buf(
        &mut self,
        toOffs: usize,
        fromPtr: &CBuf,
        fromOffs: usize,
        amount: usize,
    ) {
        cbuf_memcpy_buf(self as *mut Self, toOffs, fromPtr, fromOffs, amount)
    }

    pub unsafe fn memcpy<'a, A, B>(&mut self, toOffs: B, fromPtr: A, fromOffs: B, amount: B)
    where
        A: ToMemcpyOptions<'a>,
        B: Into<usize>,
    {
        match fromPtr.to_memcpy_option() {
            MemcpyOptions::Buf(a) => {
                self.memcpy_buf(toOffs.into(), &a, fromOffs.into(), amount.into())
            }
            MemcpyOptions::BufRef(a) => {
                self.memcpy_buf(toOffs.into(), a, fromOffs.into(), amount.into())
            }
            MemcpyOptions::Char(a) => {
                self.memcpy_char(toOffs.into(), a, fromOffs.into(), amount.into())
            }
        }
    }
    pub unsafe fn index(&mut self, idx: u16) -> &'static c_char {
        cbuf_index(self as *mut Self, idx.into()).as_ref().unwrap()
    }
    pub unsafe fn index_mut(&mut self, idx: u16) -> &'static mut c_char {
        cbuf_index(self as *mut Self, idx.into()).as_mut().unwrap()
    }
    pub unsafe fn buf(&mut self, offs: Option<usize>, amount: Option<usize>) -> &mut [i8] {
        let am = match amount {
            Some(a) => a,
            None => self.size(),
        };
        let b = match offs {
            Some(offs) => match amount {
                Some(amount) => cbuf_buf_w_both(self as *mut Self, offs, amount),
                None => cbuf_buf_w_offs(self as *mut Self, offs),
            },
            None => match amount {
                Some(amount) => cbuf_buf_w_amount(self as *mut Self, amount),
                None => cbuf_buf_none(self as *mut Self),
            },
        };
        std::slice::from_raw_parts_mut(b, am)
    }
    pub unsafe fn xornstr<'a, A, B>(&mut self, dstOffs: B, src: A, srcOffs: B, amount: B)
    where
        A: ToMemcpyOptions<'a>,
        B: Into<usize>,
    {
        match src.to_memcpy_option() {
            MemcpyOptions::Buf(a) => {
                self.xornstr_buf(dstOffs.into(), &a, srcOffs.into(), amount.into())
            }
            MemcpyOptions::BufRef(a) => {
                self.xornstr_buf(dstOffs.into(), a, srcOffs.into(), amount.into())
            }
            MemcpyOptions::Char(a) => {
                self.xornstr_char(dstOffs.into(), a, srcOffs.into(), amount.into())
            }
        }
    }
    pub unsafe fn xornstr_char(
        &mut self,
        dstOffs: usize,
        src: *const c_char,
        srcOffs: usize,
        amount: usize,
    ) {
        cbuf_xornstr_char(self as *mut Self, dstOffs, src, srcOffs, amount)
    }
    pub unsafe fn xornstr_buf(
        &mut self,
        dstOffs: usize,
        src: &CBuf,
        srcOffs: usize,
        amount: usize,
    ) {
        cbuf_xornstr_buf(self as *mut Self, dstOffs, src, srcOffs, amount)
    }
    pub unsafe fn shiftnstr(&mut self, dstOffs: usize, amount: u16, shiftAmount: u16) {
        cbuf_shiftnstr(
            self as *mut Self,
            dstOffs,
            amount.into(),
            shiftAmount.into(),
        )
    }
    pub unsafe fn size(&mut self) -> usize {
        cbuf_size(self as *mut Self)
    }
    pub unsafe fn int16at(&mut self, offs: usize) -> c_short {
        cbuf_int16at(self as *mut Self, offs)
    }
    pub unsafe fn int32at(&mut self, offs: usize) -> c_int {
        cbuf_int32at(self as *mut Self, offs)
    }
    pub unsafe fn uint16at(&mut self, offs: usize) -> c_ushort {
        cbuf_uint16at(self as *mut Self, offs)
    }
    pub unsafe fn uint32at(&mut self, offs: usize) -> c_int {
        cbuf_uint32at(self as *mut Self, offs)
    }
    pub unsafe fn hasdata(&mut self, offs: usize, amount: usize) -> bool {
        cbuf_hasdata(self as *mut Self, offs, amount)
    }
    pub unsafe fn debug_print(&mut self) {
        cbuf_debug_print(self as *mut Self)
    }
}

enum MemcpyOptions<'a> {
    Buf(CBuf),
    BufRef(&'a CBuf),
    Char(*const c_char),
}

trait ToMemcpyOptions<'a> {
    fn to_memcpy_option(self) -> MemcpyOptions<'a>;
}

impl<'a> ToMemcpyOptions<'a> for CBuf {
    fn to_memcpy_option(self) -> MemcpyOptions<'a> {
        MemcpyOptions::Buf(self)
    }
}
impl<'a> ToMemcpyOptions<'a> for &'a CBuf {
    fn to_memcpy_option(self) -> MemcpyOptions<'a> {
        MemcpyOptions::BufRef(self)
    }
}

impl<'a> ToMemcpyOptions<'a> for *const c_char {
    fn to_memcpy_option(self) -> MemcpyOptions<'a> {
        MemcpyOptions::Char(self)
    }
}

impl<'a> ToMemcpyOptions<'a> for &'a [i8] {
    fn to_memcpy_option(self) -> MemcpyOptions<'a> {
        MemcpyOptions::Char(self.as_ptr())
    }
}
impl<'a> ToMemcpyOptions<'a> for &'a [u8] {
    fn to_memcpy_option(self) -> MemcpyOptions<'a> {
        MemcpyOptions::Char(CStr::from_bytes_until_nul(self).unwrap().as_ptr())
    }
}
impl<'a> ToMemcpyOptions<'a> for &'a mut [i8] {
    fn to_memcpy_option(self) -> MemcpyOptions<'a> {
        MemcpyOptions::Char(self.as_ptr())
    }
}
impl<'a> ToMemcpyOptions<'a> for &'a mut [u8] {
    fn to_memcpy_option(self) -> MemcpyOptions<'a> {
        MemcpyOptions::Char(CStr::from_bytes_until_nul(self).unwrap().as_ptr())
    }
}
impl Drop for CBuf {
    fn drop(&mut self) {
        unsafe { cbuf_drop(self as *mut Self) }
    }
}

impl<Idx> Index<Idx> for CBuf
where
    Idx: Into<i32>,
{
    type Output = c_char;

    fn index(&self, index: Idx) -> &Self::Output {
        unsafe {
            cbuf_index(self as *const Self, index.into() as u16)
                .as_ref()
                .unwrap()
        }
    }
}

impl<Idx> IndexMut<Idx> for CBuf
where
    Idx: Into<i32>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        unsafe {
            cbuf_index(self as *const Self, index.into() as u16)
                .as_mut()
                .unwrap()
        }
    }
}
extern "C" {
    fn cbuf_new_with_size(inSize: usize) -> CBuf;
    fn cbuf_new_with_uhh(startOffs: usize, amount: usize) -> CBuf;
    fn cbuf_drop(cbuf: *mut CBuf);
    fn cbuf_resize(cbuf: *mut CBuf, inSize: usize);
    fn cbuf_memcpy_char(
        cbuf: *mut CBuf,
        toOffs: usize,
        fromPtr: *const c_char,
        fromOffs: usize,
        amount: usize,
    );
    fn cbuf_memcpy_buf(
        cbuf: *mut CBuf,
        toOffs: usize,
        fromPtr: &CBuf,
        fromOffs: usize,
        amount: usize,
    );
    fn cbuf_index(cbuf: *const CBuf, idx: u16) -> *mut c_char;
    fn cbuf_buf_none(cbuf: *mut CBuf) -> *mut c_char;
    fn cbuf_buf_w_offs(cbuf: *mut CBuf, offs: usize) -> *mut c_char;
    fn cbuf_buf_w_amount(cbuf: *mut CBuf, amount: usize) -> *mut c_char;

    fn cbuf_buf_w_both(cbuf: *mut CBuf, offs: usize, amount: usize) -> *mut c_char;

    fn cbuf_xornstr_char(
        cbuf: *mut CBuf,
        dstOffs: usize,
        src: *const c_char,
        srcOffs: usize,
        amount: usize,
    );
    fn cbuf_xornstr_buf(cbuf: *mut CBuf, dstOffs: usize, src: &CBuf, srcOffs: usize, amount: usize);
    fn cbuf_shiftnstr(cbuf: *mut CBuf, dstOffs: usize, amount: u16, shiftAmount: u16);
    fn cbuf_size(cbuf: *mut CBuf) -> usize;
    fn cbuf_int16at(cbuf: *mut CBuf, offs: usize) -> c_short;
    fn cbuf_int32at(cbuf: *mut CBuf, offs: usize) -> c_int;
    fn cbuf_uint16at(cbuf: *mut CBuf, offs: usize) -> c_ushort;
    fn cbuf_uint32at(cbuf: *mut CBuf, offs: usize) -> c_int;
    fn cbuf_hasdata(cbuf: *mut CBuf, offs: usize, amount: usize) -> bool;
    fn cbuf_debug_print(cbuf: *mut CBuf);
    //fn cbuf_set_to(CBuf &inTemplate) -> CBuf;
}
