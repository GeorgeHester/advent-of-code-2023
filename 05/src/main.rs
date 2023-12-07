use std::fs;
use std::io;
use std::path;

enum Section {
    Seed,
    Map,
}

enum LineOperation {
    Skip,
    Operate,
    SkipNewLevel,
}

struct Seed {
    id: u128,
    levels: Vec<u128>,
}

struct Range {
    input_start: u128,
    input_end: u128,
    output_start: u128,
}

fn read_file<P: AsRef<path::Path>>(file_name: P) -> io::Lines<io::BufReader<fs::File>> {
    let file: fs::File = match fs::File::open(file_name) {
        Ok(file) => file,
        Err(_) => panic!("Error: Failed to open input file"),
    };

    return io::BufRead::lines(io::BufReader::new(file));
}

fn parse_seed_ids(input: String) -> Vec<u128> {
    let mut output: Vec<u128> = Vec::new();
    let colon_index = input.find(":").unwrap();

    (&input[colon_index + ":".len()..input.len()].trim())
        .split_whitespace()
        .for_each(|seed_id_string| output.push(seed_id_string.parse::<u128>().unwrap()));

    return output;
}

fn parse_range(input: String) -> Range {
    let values: Vec<u128> = input
        .trim()
        .split_whitespace()
        .map(|value| value.parse::<u128>().unwrap())
        .collect();

    return Range {
        input_start: values[1],
        input_end: values[1] + values[2] - 1,
        output_start: values[0],
    };
}

fn determine_operation(input: &String) -> LineOperation {
    if input.len() == 0 {
        return LineOperation::SkipNewLevel;
    }

    if input.contains("map") {
        return LineOperation::Skip;
    }

    return LineOperation::Operate;
}

fn print_seeds(seeds: Vec<Seed>) {
    for seed in seeds {
        println!("{}", seed.id);

        for level in seed.levels {
            println!("-> {}", level);
        }
    }
}

fn print_levels(levels: Vec<Vec<Range>>) {
    let mut level_index = 0;
    for level in levels {
        for range in level {
            println!(
                "{} {} {} {}",
                level_index, range.input_start, range.input_end, range.output_start
            );
        }
        level_index += 1;
    }
}

fn calculate_range_output(input: u128, range: &Range) -> u128 {
    return range.output_start + (input - range.input_start);
}

fn determine_range(input: u128, ranges: &Vec<Range>) -> u128 {
    for range in ranges {
        if input < range.input_start || input > range.input_end {
            continue;
        }

        return calculate_range_output(input, &range);
    }

    return input;
}

fn determine_levels(seeds: &mut Vec<Seed>, levels: &Vec<Vec<Range>>) {
    for seed in seeds {
        seed.levels.push(determine_range(seed.id, &levels[0]));

        for level_index in 1..levels.len() {
            seed.levels.push(determine_range(
                seed.levels[level_index - 1],
                &levels[level_index],
            ));
        }
    }
}

fn main() {
    let lines = read_file("input.txt");

    let mut seeds: Vec<Seed> = Vec::new();
    let mut section = Section::Seed;
    let mut levels: Vec<Vec<Range>> = Vec::new();
    let mut level: Vec<Range> = Vec::new();

    for line_result in lines {
        let mut line = line_result.unwrap();

        match section {
            Section::Seed => {
                if line.len() == 0 {
                    section = Section::Map;
                    continue;
                }

                let seed_ids = parse_seed_ids(line);

                for seed_id in seed_ids {
                    seeds.push(Seed {
                        id: seed_id,
                        levels: Vec::new(),
                    })
                }
            }
            Section::Map => match determine_operation(&line) {
                LineOperation::SkipNewLevel => {
                    levels.push(level);
                    level = Vec::new();
                    continue;
                }
                LineOperation::Skip => {
                    continue;
                }
                LineOperation::Operate => {
                    level.push(parse_range(line));
                }
            },
        }
    }

    levels.push(level);

    determine_levels(&mut seeds, &levels);

    // print_seeds(seeds);
    // print_levels(levels);

    let mut lowest_location: u128 = u128::MAX;

    for seed in seeds {
        if lowest_location > seed.levels[seed.levels.len() - 1] {
            lowest_location = seed.levels[seed.levels.len() - 1]
        }
    }

    println!("Lowest Location: {}", lowest_location);
}
