use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction
{
    Left,
    Right,
}

pub fn direction_from_character(character: &char) -> Direction
{
    match character
    {
        'L' => return Direction::Left,
        'R' => return Direction::Right,
        _ => return Direction::Right,
    }
}
