use std::collections;

use crate::direction;

pub fn parse_directions(input: &String) -> Vec<direction::Direction>
{
    return input
        .chars()
        .map(|character| direction::direction_from_character(&character))
        .collect();
}

pub fn parse_node(input: &String) -> (String, String, String)
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

pub fn traverse(
    start_node: &String,
    end_node: &String,
    directions: &Vec<direction::Direction>,
    map: &collections::HashMap<String, (String, String)>,
) -> u64
{
    let mut count: u64 = 0;
    let mut node: String = String::from(start_node);

    loop
    {
        let next: &(String, String) = map.get(&node).unwrap();
        let current_direction: &direction::Direction =
            &directions[(count % directions.len() as u64) as usize];

        match current_direction
        {
            direction::Direction::Left => node = String::from(&next.0),
            direction::Direction::Right => node = String::from(&next.1),
        }

        count += 1;

        if node.ends_with(end_node)
        {
            break;
        }
    }

    return count;
}
