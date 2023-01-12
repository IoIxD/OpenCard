use super::page::Page;

pub struct List<'a> {
    id: u32,
    pages: &'a [&'a Page<'a>],
    unk1: u16,
    checksum: u32,
    entry_num: u32,
}