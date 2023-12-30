use std::collections;

mod direction;
mod file;

fn parse_directions(input: &String) -> Vec<direction::Direction>
{
    return input
        .chars()
        .map(|character| direction::direction_from_character(&character))
        .collect();
}

fn parse_node(input: &String) -> (String, String, String)
{
    let input_split: Vec<&str> = input.split("=").collect();
    let node: String = input_split[0].trim().to_string();

    let input_right: Vec<String> = input_split[1]
        .trim()
        .split(",")
        .map(|value| value.trim().replace("(", "").replace(")", ""))
        .collect();

    return (node, input_right[0].to_string(), input_right[1].to_string());
}

fn traverse_map(
    start: &String,
    end: &String,
    directions: &Vec<direction::Direction>,
    map: &collections::HashMap<String, (String, String)>,
) -> usize
{
    let mut count: usize = 0;
    let mut node: String = String::from(start);

    loop
    {
        let next: &(String, String) = map.get(&node).unwrap();
        let current_direction: &direction::Direction = &directions[count % directions.len()];

        match current_direction
        {
            direction::Direction::Left => node = String::from(&next.0),
            direction::Direction::Right => node = String::from(&next.1),
        }

        count += 1;

        if node == String::from(end)
        {
            break;
        }
    }

    return count;
}

fn main()
{
    let lines: Vec<String> = file::read_file("input.txt")
        .map(|value| value.unwrap())
        .collect();

    if lines.len() < 3
    {
        return;
    }

    let directions: Vec<direction::Direction> = parse_directions(&lines[0]);

    let mut map: collections::HashMap<String, (String, String)> = collections::HashMap::new();

    for index in 2..lines.len()
    {
        let (node, left, right) = parse_node(&lines[index]);
        map.insert(node, (left, right));
    }

    println!(
        "Count: {}",
        traverse_map(&"AAA".to_string(), &"ZZZ".to_string(), &directions, &map)
    )
}
