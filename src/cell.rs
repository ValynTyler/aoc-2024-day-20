use core::fmt::Debug;
use std::fmt::Display;

use colored::Colorize;

pub struct NonMazeElementError;

pub enum Cell {
    Start,
    End,
    Track,
    Wall,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Start => write!(f, "{}", 'S'),
            Cell::End   => write!(f, "{}", 'E'),
            Cell::Track => write!(f, "{}", '.'),
            Cell::Wall  => write!(f, "{}", '#'),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Start => write!(f, "{}", "󰐃".green()),
            Cell::End   => write!(f, "{}", "".purple()),
            Cell::Track => write!(f, "{}", ".".bright_black()),
            Cell::Wall  => write!(f, "{}", '#'),
        }
    }
}

impl TryFrom::<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            '.' => Ok(Self::Track),
            '#' => Ok(Self::Wall),
            _ => Err(()),
        }
    }
}
