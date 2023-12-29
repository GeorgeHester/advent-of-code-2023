mod card;
mod file;
mod hand;

fn main()
{
    let lines = file::read_file("input.txt");
    let mut hands: Vec<(hand::Hand, usize)> = Vec::new();

    lines.for_each(|line_result| {
        let line = line_result.unwrap();
        let line_split: Vec<&str> = line.split_whitespace().collect();

        let hand = hand::hand_from_string(&line_split[0].to_string());
        let bid: usize = line_split[1].parse::<usize>().unwrap();

        hands.push((hand, bid));
    });

    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands.reverse();

    let mut output: usize = 0;

    for (index, hand) in hands.iter().enumerate()
    {
        output += (index + 1) * hand.1;

        //println!("{} {}", index + 1, hand.1);
    }

    println!("Total Winnings: {}", output);
}
