use super::card::Card;

pub struct Page<'a> {
    cards: &'a [&'a Card<'a>],
}