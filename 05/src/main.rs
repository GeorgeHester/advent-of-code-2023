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

struct Range {
    input_start: u64,
    input_end: u64,
    output_start: u64,
}

fn read_file<P: AsRef<path::Path>>(file_name: P) -> io::Lines<io::BufReader<fs::File>> {
    let file: fs::File = match fs::File::open(file_name) {
        Ok(file) => file,
        Err(_) => panic!("Error: Failed to open input file"),
    };

    return io::BufRead::lines(io::BufReader::new(file));
}

fn parse_seed_ids(input: String) -> Vec<u64> {
    let mut values: Vec<u64> = Vec::new();
    let mut output: Vec<u64> = Vec::new();
    let colon_index = input.find(":").unwrap();

    (&input[colon_index + ":".len()..input.len()].trim())
        .split_whitespace()
        .for_each(|seed_id_string| values.push(seed_id_string.parse::<u64>().unwrap()));

    for value_index in (0..values.len()).step_by(2) {
        for index in 0..values[value_index + 1] {
            output.push(values[value_index] + index);
        }
    }

    return output;
}

fn parse_range(input: String) -> Range {
    let values: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
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

fn calculate_range_output(input: u64, range: &Range) -> u64 {
    return range.output_start + (input - range.input_start);
}

fn determine_range(input: u64, ranges: &Vec<Range>) -> u64 {
    for range in ranges {
        if input < range.input_start || input > range.input_end {
            continue;
        }

        return calculate_range_output(input, &range);
    }

    return input;
}

fn determine_last_level(start_value: u64, levels: &Vec<Vec<Range>>) -> u64 {
    let mut previous_level_value: u64 = start_value;
    for level_index in 0..levels.len() {
        previous_level_value = determine_range(previous_level_value, &levels[level_index]);
    }

    return previous_level_value;
}

fn main() {
    let lines = read_file("input.txt");

    let mut seeds: Vec<u64> = Vec::new();
    let mut section = Section::Seed;
    let mut levels: Vec<Vec<Range>> = Vec::new();
    let mut level: Vec<Range> = Vec::new();

    for line_result in lines {
        let line = line_result.unwrap();

        match section {
            Section::Seed => {
                if line.len() == 0 {
                    section = Section::Map;
                    continue;
                }

                let seed_ids = parse_seed_ids(line);

                for seed_id in seed_ids {
                    seeds.push(seed_id);
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

    let mut lowest_location: u64 = u64::MAX;

    for seed in seeds {
        let new_location: u64 = determine_last_level(seed, &levels);

        if lowest_location > new_location {
            lowest_location = new_location
        }
    }

    println!("Lowest Location: {}", lowest_location);
}
