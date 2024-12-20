pub struct NonMazeElementError;

pub enum Cell {
    Start,
    End,
    Track,
    Wall,
}

impl TryFrom::<char> for Cell {
    type Error = NonMazeElementError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            '.' => Ok(Self::Track),
            '#' => Ok(Self::Wall),
            _ => Err(NonMazeElementError),
        }
    }
}
