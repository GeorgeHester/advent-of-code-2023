use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction
{
    Left,
    Right,
}

impl fmt::Debug for Direction
{
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Direction::Left => format.pad("Left"),
            Direction::Right => format.pad("Right"),
        }
    }
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
