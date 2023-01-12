use super::background::Background;
use super::list::List;
use super::card::Card;
use super::font::Font;
use super::style::Style;

pub enum StackFormat {
    NotHyperCard,
    PreReleaseHyperCard1x,
    HyperCard1x,
    PreReleaseHyperCard2x,
    HyperCard2x,
}

pub struct CardStack<'a> {
    size: u32,
    format: StackFormat,
    data_fork_size: u32,
    stack_size: u32,
    unk1: u32,
    max_of_prev_value: u32,     // wat

    background_num: u32,
    first_background: &'a Background<'a>,

    card_num: u32,
    first_card: &'a Card<'a>,

    list: &'a List<'a>,

    password_hash: u32,

    user_level: u16,

    prot_flags: u16,

    version_at_creation: u32,
    version_at_last_compacting: u32,
    version_at_last_modification_since_compacting: u32,
    version_at_last_modification: u32,

    checksum: u32,

    marked_card_num: u32,
    top_of_card_window: u16,
    left_of_card_window: u16,
    bottom_of_card_window: u16,
    right_of_card_window: u16,
    top_of_screen: u16,
    left_of_screen: u16,
    bottom_of_screen: u16,
    right_of_screen: u16,
    x_coord_scroll: u16,
    y_coord_scroll: u16,

    unk2: u16,

    font_table: &'a [&'a Font<'a>],
    style_table: &'a [&'a Style<'a>],

    height: u16,
    width: u16,

    unk3: u16,
    unk4: u16,

    patterns: &'a [u8],
    script: &'a str
}