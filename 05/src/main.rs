use std::cmp;
use std::collections;
use std::fs;
use std::io;
use std::path;

enum Section
{
    Seed,
    Map,
}

enum LineOperation
{
    Skip,
    Operate,
    NewLevel,
}

struct Range
{
    start: u64,
    end: u64,
}

struct RangeWithOutput
{
    start: u64,
    end: u64,
    output: u64,
}

fn read_file<P: AsRef<path::Path>>(file_name: P) -> io::Lines<io::BufReader<fs::File>>
{
    let file: fs::File = match fs::File::open(file_name)
    {
        Ok(file) => file,
        Err(_) => panic!("Error: Failed to open input file"),
    };

    return io::BufRead::lines(io::BufReader::new(file));
}

fn determine_operation(input: &String) -> LineOperation
{
    if input.len() == 0
    {
        return LineOperation::NewLevel;
    }

    if input.contains("map")
    {
        return LineOperation::Skip;
    }

    return LineOperation::Operate;
}

fn parse_seed_ranges(input: &String) -> Vec<Range>
{
    let mut values: Vec<u64> = Vec::new();
    let mut output: Vec<Range> = Vec::new();

    let colon_index = input.find(":").unwrap();

    (&input[colon_index + ":".len()..input.len()].trim())
        .split_whitespace()
        .for_each(|seed_id_string| values.push(seed_id_string.parse::<u64>().unwrap()));

    for value_index in (0..values.len()).step_by(2)
    {
        output.push(Range {
            start: values[value_index],
            end: values[value_index] + values[value_index + 1],
        });
    }

    return output;
}

fn parse_level_ranges(input: &String) -> RangeWithOutput
{
    let values: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect();

    return RangeWithOutput {
        start: values[1],
        end: values[1] + values[2],
        output: values[0],
    };
}

/*
fn determine_last_level(inital: u64, maps: &Vec<collections::HashMap<u64, u64>>) -> u64
{
    let mut previous_map_value: u64 = inital;

    for map_index in 0..maps.len()
    {
        if maps[map_index].contains_key(&previous_map_value)
        {
            previous_map_value = maps[map_index][&previous_map_value];
        }
    }

    return previous_map_value;
}
*/

fn print_ranges(ranges: &Vec<Range>)
{
    for range in ranges
    {
        println!("{} {}", range.start, range.end);
    }
}

fn print_levels(levels: &Vec<Vec<RangeWithOutput>>)
{
    let mut level_index = 0;
    for level in levels
    {
        println!("Level {}", level_index);
        for range in level
        {
            println!("{} {} {}", range.start, range.end, range.output);
        }
        level_index += 1;
    }
}

fn main()
{
    let lines = read_file("input.txt");

    let mut section = Section::Seed;
    let mut seed_ranges: Vec<Range> = Vec::new();
    let mut levels: Vec<Vec<RangeWithOutput>> = Vec::new();
    let mut level: Vec<RangeWithOutput> = Vec::new();

    for line_result in lines
    {
        let line = line_result.unwrap();

        match section
        {
            Section::Seed =>
            {
                if line.len() == 0
                {
                    section = Section::Map;
                    continue;
                }

                seed_ranges = parse_seed_ranges(&line);
            }
            Section::Map => match determine_operation(&line)
            {
                LineOperation::NewLevel =>
                {
                    levels.push(level);
                    level = Vec::new();
                    continue;
                }
                LineOperation::Skip =>
                {
                    continue;
                }
                LineOperation::Operate =>
                {
                    level.push(parse_level_ranges(&line));
                }
            },
        }
    }

    levels.push(level);

    //print_ranges(&seed_ranges);
    //print_levels(&levels);

    for level_ranges in &levels
    {
        let mut new_seed_ranges: Vec<Range> = Vec::new();

        while seed_ranges.len() > 0
        {
            let seed_range = seed_ranges.pop().unwrap();
            let mut added: bool = false;

            for level_range in level_ranges
            {
                let overlap_start: u64 = cmp::max(seed_range.start, level_range.start);
                let overlap_end: u64 = cmp::min(seed_range.end, level_range.end);

                if overlap_start < overlap_end
                {
                    new_seed_ranges.push(Range {
                        start: overlap_start - level_range.start + level_range.output,
                        end: overlap_end - level_range.start + level_range.output,
                    });

                    if overlap_start > seed_range.start
                    {
                        seed_ranges.push(Range {
                            start: seed_range.start,
                            end: overlap_start,
                        });
                    }

                    if overlap_end < seed_range.end
                    {
                        seed_ranges.push(Range {
                            start: overlap_end,
                            end: seed_range.end,
                        });
                    }
                    added = true;
                }
            }

            if !added
            {
                new_seed_ranges.push(seed_range);
            }
        }

        seed_ranges = new_seed_ranges;
    }

    println!(
        "Lowest Location: {}",
        seed_ranges
            .iter()
            .min_by(|range_x, range_y| range_x.start.cmp(&range_y.start))
            .unwrap()
            .start
    );

    //print_ranges(&seed_ranges);
    /*  let mut lowest_location: u64 = u64::MAX;

    for seed in seeds
    {
        let new_location: u64 = determine_last_level(seed, &level_maps);

        if lowest_location > new_location
        {
            lowest_location = new_location
        }
    }

    println!("Lowest Location: {}", lowest_location);*/
}
