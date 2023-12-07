use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{newline, space0, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, game_list) = parse_input(input).map_err(|e| AocError::ParseError(e.to_string()))?;

    // dbg!(&game_list, &game_list.len());
    let sum_of_scores = game_list
        .iter()
        .fold(0, |score, (winning_numbers, available_numbers)| {
            let mut game_score = 0;
            for num in winning_numbers {
                if available_numbers.contains_key(num) {
                    if game_score == 0 {
                        game_score = 1
                    } else {
                        game_score *= 2
                    }
                }
            }
            score + game_score
        });

    Ok(sum_of_scores)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<u8>, HashMap<u8, ()>)>> {
    separated_list1(newline, line)(input)
}

pub fn line(input: &str) -> IResult<&str, (Vec<u8>, HashMap<u8, ()>)> {
    let (input, _) = take_until1(": ")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, (winning_numbers, game_numbers)) = separated_pair(
        separated_list1(space1, nom::character::complete::u8),
        delimited(space0, tag("|"), space0),
        nom::combinator::map(separated_list1(space1, nom::character::complete::u8), |v| {
            v.iter().map(|n| (*n, ())).collect::<HashMap<u8, ()>>()
        }),
    )(input)?;

    Ok((input, (winning_numbers, game_numbers)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(13, process(input)?);
        Ok(())
    }
}
