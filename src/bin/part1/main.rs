use aoc_2024_day_20::maze::{BadDataError, Maze};

fn main() -> Result<(), BadDataError> {
    println!("Working on part 1...");

    let input_string = include_str!("../../../input/example.txt");
    let maze = Maze::try_from(input_string)?;
    println!("{}", maze);

    Ok(())
}
