use std::cmp;

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
    pub cards: Vec<u8>,
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
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand
{
    fn eq(&self, other: &Self) -> bool
    {
        self.cards.cmp(&other.cards) == cmp::Ordering::Equal
    }
}
