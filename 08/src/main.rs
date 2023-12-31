use std::collections;

mod direction;
mod file;
mod map;
mod math;

fn main()
{
    let lines: Vec<String> = file::read_file("input.txt")
        .map(|value| value.unwrap())
        .collect();

    if lines.len() < 3
    {
        return;
    }

    let directions: Vec<direction::Direction> = map::parse_directions(&lines[0]);

    let mut map: collections::HashMap<String, (String, String)> = collections::HashMap::new();

    for index in 2..lines.len()
    {
        let (node, left, right) = map::parse_node(&lines[index]);
        map.insert(node, (left, right));
    }

    let start_nodes: Vec<String> = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|value| String::from(value))
        .collect();

    let mut counts: Vec<u64> = Vec::new();

    for start_node in &start_nodes
    {
        counts.push(map::traverse(
            start_node,
            &"Z".to_string(),
            &directions,
            &map,
        ));
    }

    println!("Count: {}", math::lcm_vec(counts));
}
