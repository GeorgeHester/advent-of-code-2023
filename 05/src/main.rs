use std::collections;
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
    NewLevel,
}

fn read_file<P: AsRef<path::Path>>(file_name: P) -> io::Lines<io::BufReader<fs::File>> {
    let file: fs::File = match fs::File::open(file_name) {
        Ok(file) => file,
        Err(_) => panic!("Error: Failed to open input file"),
    };

    return io::BufRead::lines(io::BufReader::new(file));
}

fn determine_operation(input: &String) -> LineOperation {
    if input.len() == 0 {
        return LineOperation::NewLevel;
    }

    if input.contains("map") {
        return LineOperation::Skip;
    }

    return LineOperation::Operate;
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

fn update_range_map(map: &mut collections::HashMap<u64, u64>, input: &String) {
    let values: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect();

    for input_value in values[1]..values[1] + values[2] {
        let output_value = values[0] + input_value - values[1];
        map.insert(input_value, output_value);
    }
}

fn determine_last_level(inital: u64, maps: &Vec<collections::HashMap<u64, u64>>) -> u64 {
    let mut previous_map_value: u64 = inital;

    for map_index in 0..maps.len() {
        if maps[map_index].contains_key(&previous_map_value) {
            previous_map_value = maps[map_index][&previous_map_value];
        }
    }

    return previous_map_value;
}

fn main() {
    let lines = read_file("input.txt");

    let mut section = Section::Seed;
    let mut seeds: Vec<u64> = Vec::new();
    let mut level_maps: Vec<collections::HashMap<u64, u64>> = Vec::new();
    let mut level_map: collections::HashMap<u64, u64> = collections::HashMap::new();

    for line_result in lines {
        let line = line_result.unwrap();

        match section {
            Section::Seed => {
                if line.len() == 0 {
                    section = Section::Map;
                    continue;
                }

                seeds = parse_seed_ids(line);
                seeds.dedup();
            }
            Section::Map => match determine_operation(&line) {
                LineOperation::NewLevel => {
                    level_maps.push(level_map);
                    level_map = collections::HashMap::new();
                    continue;
                }
                LineOperation::Skip => {
                    continue;
                }
                LineOperation::Operate => {
                    update_range_map(&mut level_map, &line);
                }
            },
        }
    }

    level_maps.push(level_map);

    let mut lowest_location: u64 = u64::MAX;

    for seed in seeds {
        let new_location: u64 = determine_last_level(seed, &level_maps);

        if lowest_location > new_location {
            lowest_location = new_location
        }
    }

    println!("Lowest Location: {}", lowest_location);
}
