use std::cmp;
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
    let mut output: Vec<Range> = Vec::new();
    let colon_index = input.find(":").unwrap();

    let values: Vec<u64> = (&input[colon_index + ":".len()..input.len()])
        .trim()
        .split_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect();

    for value_index in (0..values.len()).step_by(2)
    {
        output.push(Range {
            start: values[value_index],
            end: values[value_index] + values[value_index + 1],
        });
    }

    return output;
}

fn parse_level_range(input: &String) -> RangeWithOutput
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
*/

fn transform_range(seed_range: &Range, ranges: &Vec<RangeWithOutput>) -> Vec<Range>
{
    let mut output: Vec<Range> = Vec::new();
    let mut output_transformed: bool = false;

    for range in ranges
    {
        let overlap_start: u64 = cmp::max(seed_range.start, range.start);
        let overlap_end: u64 = cmp::min(seed_range.end, range.end);

        if overlap_start < overlap_end
        {
            output.push(Range {
                start: overlap_start - range.start + range.output,
                end: overlap_end - range.start + range.output,
            });

            if overlap_start > seed_range.start
            {
                output.append(&mut transform_range(
                    &Range {
                        start: seed_range.start,
                        end: overlap_start,
                    },
                    ranges,
                ))
            }

            if overlap_end < seed_range.end
            {
                output.append(&mut transform_range(
                    &Range {
                        start: overlap_end,
                        end: seed_range.end,
                    },
                    ranges,
                ))
            }

            output_transformed = true;
        }
    }

    if !output_transformed
    {
        output.push(Range {
            start: seed_range.start,
            end: seed_range.end,
        });
    }

    return output;
}

fn transform_output(seed_ranges: Vec<Range>, levels: &Vec<Vec<RangeWithOutput>>) -> Vec<Range>
{
    let mut output_seed_ranges: Vec<Range> = seed_ranges;

    for ranges in levels
    {
        let mut hold_seed_ranges: Vec<Range> = Vec::new();

        for seed_range in output_seed_ranges
        {
            hold_seed_ranges.append(&mut transform_range(&seed_range, &ranges));
        }

        output_seed_ranges = hold_seed_ranges;
    }

    return output_seed_ranges;
}

fn main()
{
    let lines = read_file("input.txt");

    let mut section = Section::Seed;
    let mut seed_ranges: Vec<Range> = Vec::new();
    let mut levels: Vec<Vec<RangeWithOutput>> = Vec::new();
    let mut ranges: Vec<RangeWithOutput> = Vec::new();

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
                    levels.push(ranges);
                    ranges = Vec::new();
                    continue;
                }
                LineOperation::Skip =>
                {
                    continue;
                }
                LineOperation::Operate =>
                {
                    ranges.push(parse_level_range(&line));
                }
            },
        }
    }

    levels.push(ranges);

    //print_ranges(&seed_ranges);
    //print_levels(&levels);

    /*for level_ranges in &levels
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
    }*/

    let output_seed_ranges: Vec<Range> = transform_output(seed_ranges, &levels);

    println!(
        "Lowest Location: {}",
        output_seed_ranges
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
