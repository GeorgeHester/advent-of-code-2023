use std::cmp;
use std::fs;
use std::io;
use std::iter;
use std::path;

fn read_file<P: AsRef<path::Path>>(file_name: P) -> io::Lines<io::BufReader<fs::File>>
{
    let file: fs::File = match fs::File::open(file_name)
    {
        Ok(file) => file,
        Err(_) => panic!("Error: Failed to open input file"),
    };

    return io::BufRead::lines(io::BufReader::new(file));
}

/*
struct Race
{
    length: u64,
    distance: u64,
}*/

fn parse_race_lengths(input: &String) -> Vec<u64>
{
    let lengths: Vec<u64> = input.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    return lengths;
}

fn parse_race_distances(input: &String) -> Vec<u64>
{
    let lengths: Vec<u64> = input.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    return lengths;
}

fn main()
{
    let lines = read_file("input.txt")
        .map(|value| value.unwrap())
        .collect::<Vec<String>>();

    //let mut races: Vec<Race> = Vec::new();

    if lines.len() < 2
    {
        return;
    }

    let lengths = parse_race_lengths(&lines[0]);
    let distances = parse_race_distances(&lines[1]);

    if lengths.len() != distances.len()
    {
        return;
    }

    let mut numbers_of_ways: Vec<u64> = Vec::new();

    for race in iter::zip(lengths, distances)
    {
        let (length, distance) = race;
        let mut number_of_ways = 0;

        for case in 0..length
        {
            let case_distance = (length - case) * case;

            if case_distance > distance
            {
                number_of_ways += 1;
            }
        }

        numbers_of_ways.push(number_of_ways);
    }

    println!(
        "Number Ways: {}",
        numbers_of_ways
            .iter()
            .copied()
            .reduce(|a, b| a * b)
            .unwrap()
    )
}
