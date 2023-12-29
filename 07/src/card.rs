use std::cmp;

#[derive(Eq)]
pub struct Card
{
    value: u8,
}

impl Ord for Card
{
    fn cmp(&self, other: &Self) -> cmp::Ordering
    {
        return self.value.cmp(&other.value);
    }
}

impl PartialOrd for Card
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
    {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Card
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.value == other.value;
    }
}

fn card_value_from_character(character: &char) -> u8
{
    return "123456789TJQKA".find(*character).unwrap() as u8;
}

pub fn card_from_character(character: &char) -> Card
{
    return Card {
        value: card_value_from_character(character),
    };
}
