use std::fmt::Debug;

use ndarray::Array2;
use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, pipe_map) = parse_input(input).map_err(|e| AocError::NomParseError(e.to_string()))?;
    let ((start_row, start_col), _) = pipe_map
        .indexed_iter()
        .find(|((_, _), p)| **p == Pipe::Start)
        .expect("No start");
    let num_rows = pipe_map.nrows();
    let num_cols = pipe_map.ncols();

    let directions_from_start = [
        Direction::Up,
        Direction::Right,
        Direction::Left,
        Direction::Down,
    ];

    let mut distance_map: Array2<Option<u32>> =
        Array2::from_shape_simple_fn(pipe_map.raw_dim(), || None);

    for start_direction in directions_from_start {
        let mut distance = 1;

        let Some((mut current_row, mut current_col)) =
            start_direction.get_new_index(start_row, start_col, num_rows - 1, num_cols - 1)
        else {
            continue;
        };
        let current_distance_value = distance_map
            .get_mut((current_row, current_col))
            .expect("First value doesn't exist in distance map")
            .get_or_insert(distance);
        *current_distance_value = *current_distance_value.min(&mut distance);

        let mut current_pipe_val = pipe_map
            .get((current_row, current_col))
            .expect("Invalid starting pipe");
        let mut current_direction = start_direction;
        while let Some(new_direction) = current_pipe_val.get_new_direction(&current_direction) {
            current_direction = new_direction;
            let Some((new_row, new_col)) = current_direction.get_new_index(
                current_row,
                current_col,
                num_rows - 1,
                num_cols - 1,
            ) else {
                break;
            };
            current_row = new_row;
            current_col = new_col;
            current_pipe_val = pipe_map
                .get((current_row, current_col))
                .expect("Invalid index");
            if *current_pipe_val == Pipe::Start || *current_pipe_val == Pipe::NoPipe {
                break;
            }
            distance += 1;
            let current_distance_value = distance_map
                .get_mut((current_row, current_col))
                .expect("Invalid value in distance_map")
                .get_or_insert(distance);
            *current_distance_value = *current_distance_value.min(&mut distance);
        }
    }

    let max_distance = distance_map
        .iter()
        .map(|s| s.unwrap_or(0))
        .max()
        .expect("No max value");

    Ok(max_distance)
}

pub fn parse_input(input: &str) -> IResult<&str, Array2<Pipe>> {
    let (input, pipes) = separated_list1(newline, many1(parse_pipe))(input)?;
    let num_rows = pipes.len();
    let num_cols = pipes.first().expect("empty vec").len();
    let matrix =
        Array2::from_shape_vec((num_rows, num_cols), pipes.into_iter().flatten().collect())
            .expect("Not valid matrix??");
    Ok((input, matrix))
}

pub fn parse_pipe(input: &str) -> IResult<&str, Pipe> {
    map(one_of("S.|-LJ7F"), |c| match c {
        'S' => Pipe::Start,
        '.' => Pipe::NoPipe,
        '|' => Pipe::Vertical,
        '-' => Pipe::Horizontal,
        'L' => Pipe::NorthEastBend,
        'J' => Pipe::NorthWestBend,
        '7' => Pipe::SouthWestBend,
        'F' => Pipe::SouthEastBend,
        _ => panic!("invalid symbol"),
    })(input)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn is_valid_direction(&self, pipe: &Pipe) -> bool {
        match (pipe, self) {
            (Pipe::Start, _) => true,
            (Pipe::Vertical, Direction::Down | Direction::Up)
            | (Pipe::Horizontal, Direction::Left | Direction::Right) => true,
            (Pipe::NorthEastBend, Direction::Down)
            | (Pipe::NorthEastBend, Direction::Left)
            | (Pipe::NorthWestBend, Direction::Down)
            | (Pipe::NorthWestBend, Direction::Right)
            | (Pipe::SouthEastBend, Direction::Up)
            | (Pipe::SouthEastBend, Direction::Left)
            | (Pipe::SouthWestBend, Direction::Up)
            | (Pipe::SouthWestBend, Direction::Right) => true,
            _ => false,
        }
    }

    pub fn get_new_index(
        &self,
        row: usize,
        col: usize,
        max_row: usize,
        max_col: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, col))
                }
            }
            Direction::Left => {
                if col == 0 {
                    None
                } else {
                    Some((row, col - 1))
                }
            }
            Direction::Right => {
                if col == max_col {
                    None
                } else {
                    Some((row, col + 1))
                }
            }
            Direction::Down => {
                if row == max_row {
                    None
                } else {
                    Some((row + 1, col))
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Start,
    NoPipe,
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "S"),
            Self::NoPipe => write!(f, "."),
            Self::Vertical => write!(f, "|"),
            Self::Horizontal => write!(f, "-"),
            Self::NorthEastBend => write!(f, "L"),
            Self::NorthWestBend => write!(f, "J"),
            Self::SouthWestBend => write!(f, "7"),
            Self::SouthEastBend => write!(f, "F"),
        }
    }
}

impl Pipe {
    pub fn get_new_direction(&self, start_direction: &Direction) -> Option<Direction> {
        match (self, start_direction) {
            (Pipe::Vertical, Direction::Down | Direction::Up)
            | (Pipe::Horizontal, Direction::Left | Direction::Right) => Some(*start_direction),
            (Pipe::NorthEastBend, Direction::Down) => Some(Direction::Right),
            (Pipe::NorthEastBend, Direction::Left) => Some(Direction::Up),
            (Pipe::NorthWestBend, Direction::Down) => Some(Direction::Left),
            (Pipe::NorthWestBend, Direction::Right) => Some(Direction::Up),
            (Pipe::SouthEastBend, Direction::Up) => Some(Direction::Right),
            (Pipe::SouthEastBend, Direction::Left) => Some(Direction::Down),
            (Pipe::SouthWestBend, Direction::Up) => Some(Direction::Left),
            (Pipe::SouthWestBend, Direction::Right) => Some(Direction::Down),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_with_test_input_1() -> miette::Result<()> {
        let input = include_str!("../test-input1.txt");
        assert_eq!(4, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_with_test_input_2() -> miette::Result<()> {
        let input = include_str!("../test-input2.txt");
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
