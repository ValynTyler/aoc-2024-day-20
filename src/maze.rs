use std::fmt::Display;

use utf8_box_builder::grid::Grid;

use crate::cell::{Cell, NonMazeElementError};

#[derive(Debug)]
pub struct BadDataError;

impl From::<NonMazeElementError> for BadDataError {
    fn from(_: NonMazeElementError) -> Self { Self }
}

#[derive(Debug)]
pub struct Maze(Grid<Cell>);

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom::<&str> for Maze {
    type Error = BadDataError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(Grid(value
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|token| { Cell::try_from(token) })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<Vec<_>>, _>>()
            .or(Err(BadDataError))?
        )))
    }
}
