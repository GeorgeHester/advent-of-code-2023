use std::fs;
use std::io;
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

fn parse_race_length(input: &String) -> u64
{
    let length: u64 = input.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    return length;
}

fn parse_race_distance(input: &String) -> u64
{
    let distance: u64 = input.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    return distance;
}

fn main()
{
    let lines = read_file("input.txt")
        .map(|value| value.unwrap())
        .collect::<Vec<String>>();

    if lines.len() < 2
    {
        return;
    }

    let length = parse_race_length(&lines[0]);
    let distance = parse_race_distance(&lines[1]);

    let mut number_of_ways: u64 = 0;

    for case in 0..length
    {
        let case_distance = (length - case) * case;

        if case_distance > distance
        {
            number_of_ways += 1;
        }
    }

    println!("Number Ways: {}", number_of_ways)
}
