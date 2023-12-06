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

fn main()
{
    let lines = read_file("input.txt");

    let mut total_points: u16 = 0;

    for line_result in lines
    {
        let mut line = line_result.unwrap();
        let id: u8 = get_card_id(&mut line);
        let winning_numbers: Vec<u8> = get_card_winning_numbers(&mut line);
        let mut card_numbers: Vec<u8> = get_card_numbers(&mut line);

        card_numbers.retain(|number| winning_numbers.contains(number));

        total_points += calculate_points(card_numbers.len());
    }

    println!("Total Points: {}", total_points);
}
