use std::fs;
use std::io;
use std::path;

const CARD_SEPERATOR: &str = "Card";
const COLON_SEPERATOR: &str = ":";
const BAR_SEPERATOR: &str = "|";

fn read_file<P: AsRef<path::Path>>(file_name: P) -> io::Lines<io::BufReader<fs::File>>
{
    let file: fs::File = match fs::File::open(file_name)
    {
        Ok(file) => file,
        Err(_) => panic!("Error: Failed to open input file"),
    };

    return io::BufRead::lines(io::BufReader::new(file));
}

fn get_card_id(input: &mut String) -> u8
{
    let card_index = input.find(CARD_SEPERATOR).unwrap();
    let colon_index = input.find(COLON_SEPERATOR).unwrap();

    let id: u8 = (&input[card_index + CARD_SEPERATOR.len()..colon_index].trim())
        .parse::<u8>()
        .unwrap();

    input.drain(0..colon_index + COLON_SEPERATOR.len());

    return id;
}

fn get_card_winning_numbers(input: &mut String) -> Vec<u8>
{
    let mut output: Vec<u8> = Vec::new();

    let bar_index = input.find(BAR_SEPERATOR).unwrap();
    let winning_numbers_string: &str = &input[0..bar_index].trim();

    winning_numbers_string
        .split_whitespace()
        .for_each(|winning_number_string| {
            output.push(winning_number_string.parse::<u8>().unwrap())
        });

    input.drain(0..bar_index + BAR_SEPERATOR.len());

    return output;
}

fn get_card_numbers(input: &mut String) -> Vec<u8>
{
    let mut output: Vec<u8> = Vec::new();

    let numbers_string: &str = &input.trim();

    numbers_string
        .split_whitespace()
        .for_each(|winning_number_string| {
            output.push(winning_number_string.parse::<u8>().unwrap())
        });

    return output;
}

fn calculate_points(value: usize) -> u16
{
    if value == 0
    {
        return 0;
    }

    return u16::pow(2, (value - 1) as u32);
}

struct Card
{
    id: u8,
    matches: u8,
    copies: u32,
}

fn print_cards(cards: Vec<Card>)
{
    for card in cards
    {
        println!("{} {} {}", card.id, card.matches, card.copies);
    }
}

fn main()
{
    let lines = read_file("input.txt");

    let mut total_points: u16 = 0;
    let mut total_scratchcards: u32 = 0;

    let mut cards: Vec<Card> = Vec::new();

    for line_result in lines
    {
        let mut line = line_result.unwrap();
        let id: u8 = get_card_id(&mut line);
        let winning_numbers: Vec<u8> = get_card_winning_numbers(&mut line);
        let mut card_numbers: Vec<u8> = get_card_numbers(&mut line);

        card_numbers.retain(|number| winning_numbers.contains(number));

        cards.push(Card {
            id: id,
            matches: card_numbers.len() as u8,
            copies: 1,
        });
        total_points += calculate_points(card_numbers.len());
    }

    for card_index in 0..cards.len()
    {
        let card_matches = cards[card_index].matches;
        let card_copes = cards[card_index].copies;

        for _ in 0..card_copes
        {
            for update_index in card_index + 1..card_index + 1 + (card_matches as usize)
            {
                cards[update_index].copies += 1;
            }
        }
    }

    for card in cards.iter()
    {
        total_scratchcards += card.copies as u32;
    }

    // sprint_cards(cards);

    println!("Total Points: {}", total_points);
    println!("Total Scratchcards: {}", total_scratchcards);
}
