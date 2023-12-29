use std::collections::HashMap;

mod file;
mod hand;

const STENGTH: &str = "123456789TJQKA";

fn get_card_strength(card: &char) -> u8
{
    return STENGTH.find(*card).unwrap() as u8;
}

fn get_hand_strength(hand: &String) -> Vec<u8>
{
    let mut output: Vec<u8> = Vec::new();

    for card in hand.chars()
    {
        output.push(get_card_strength(&card));
    }

    return output;
}

fn map_hand(hand: &String) -> HashMap<char, u8>
{
    let mut map: HashMap<char, u8> = HashMap::new();

    for card in hand.chars()
    {
        let map_entry = map.entry(card).or_insert(0);
        *map_entry += 1;
    }

    return map;
}

fn type_hand(hand: &Vec<u8>) -> hand::HandType
{
    match hand[0]
    {
        5 => return hand::HandType::FiveOfAKind,
        4 => return hand::HandType::FourOfAKind,
        3 => match hand[1]
        {
            2 => return hand::HandType::FullHouse,
            _ => return hand::HandType::ThreeOfAKind,
        },
        2 => match hand[1]
        {
            2 => return hand::HandType::TwoPair,
            _ => return hand::HandType::OnePair,
        },
        _ => return hand::HandType::HighCard,
    }
}

fn parse_hand(hand: &String) -> hand::Hand
{
    let map = map_hand(hand);
    let mut card_counts = map.values().map(|value| *value).collect::<Vec<u8>>();
    card_counts.sort_by(|a, b| b.cmp(a));

    let output: hand::Hand = hand::Hand {
        hand_type: type_hand(&card_counts),
        cards: get_hand_strength(hand),
    };

    return output;
}

fn main()
{
    let lines = file::read_file("input.txt");

    let mut hands: Vec<(hand::Hand, usize)> = Vec::new();

    for line_result in lines
    {
        let line = line_result.unwrap();
        let line_split: Vec<&str> = line.split_whitespace().collect();

        let hand = parse_hand(&line_split[0].to_string());
        let bid: usize = line_split[1].parse::<usize>().unwrap();

        hands.push((hand, bid));
    }

    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands.reverse();

    let mut output: usize = 0;

    for (index, hand) in hands.iter().enumerate()
    {
        output += (index + 1) * hand.1;

        println!("{} {}", index + 1, hand.1);
    }

    println!("Total Winnings: {}", output);
}
