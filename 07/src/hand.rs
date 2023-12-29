use std::cmp;
use std::collections;

use crate::card;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType
{
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq)]
pub struct Hand
{
    pub hand_type: HandType,
    pub cards: Vec<card::Card>,
}

impl Ord for Hand
{
    fn cmp(&self, other: &Self) -> cmp::Ordering
    {
        let type_order = self.hand_type.cmp(&other.hand_type);

        if type_order != cmp::Ordering::Equal
        {
            return type_order;
        }

        return other.cards.cmp(&self.cards);
    }
}

impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
    {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Hand
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.cards.cmp(&other.cards) == cmp::Ordering::Equal;
    }
}

fn hand_type_from_string(string: &String) -> HandType
{
    let mut multiples_map: collections::HashMap<char, u8> = collections::HashMap::new();

    string.chars().for_each(|character| {
        let multiples_map_entry = multiples_map.entry(character).or_insert(0);
        *multiples_map_entry += 1;
    });

    let mut multiples: Vec<u8> = multiples_map.values().map(|value| *value).collect();
    multiples.sort_by(|a, b| b.cmp(a));

    match multiples[0]
    {
        5 => return HandType::FiveOfAKind,
        4 => return HandType::FourOfAKind,
        3 => match multiples[1]
        {
            2 => return HandType::FullHouse,
            _ => return HandType::ThreeOfAKind,
        },
        2 => match multiples[1]
        {
            2 => return HandType::TwoPair,
            _ => return HandType::OnePair,
        },
        _ => return HandType::HighCard,
    }
}

fn hand_cards_from_string(string: &String) -> Vec<card::Card>
{
    return string
        .chars()
        .map(|value| card::card_from_character(&value))
        .collect();
}

pub fn hand_from_string(string: &String) -> Hand
{
    return Hand {
        hand_type: hand_type_from_string(string),
        cards: hand_cards_from_string(string),
    };
}
