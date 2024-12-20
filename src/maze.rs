use utf8_box_builder::grid::Grid;

use crate::cell::Cell;

pub struct Maze(Grid<Cell>);

impl From::<&str> for Maze {
    fn from(value: &str) -> Self {
        todo!()
    }
}
