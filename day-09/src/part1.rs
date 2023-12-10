use itertools::Itertools;
use nom::{
    character::complete::{newline, space1},
    multi::separated_list1,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i64, AocError> {
    let (_, values) = separated_list1(newline, parse_line)(input)
        .map_err(|e| AocError::NomParseError(e.to_string()))?;

    let next_value_sum = values
        .into_iter()
        .map(|value_list| {
            let mut stack = vec![*value_list.last().unwrap()];
            let mut current_stack = value_list;
            while !current_stack.iter().all(|v| *v == 0) {
                current_stack = current_stack
                    .iter()
                    .tuple_windows()
                    .map(|(l, r)| r - l)
                    .collect();
                stack.push(*current_stack.last().unwrap());
            }
            let last_number = stack.iter().fold(0, |number_to_add, v| number_to_add + *v);
            last_number
        })
        .sum();
    Ok(next_value_sum)
}

pub fn parse_line(input: &str) -> nom::IResult<&str, Vec<i64>> {
    separated_list1(space1, nom::character::complete::i64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(114, process(input)?);
        Ok(())
    }
}
