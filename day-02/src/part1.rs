use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::{alpha1, newline, space0, u32},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

const NUM_BLUE: u32 = 14;
const NUM_RED: u32 = 12;
const NUM_GREEN: u32 = 13;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, games) = parse_input(input).unwrap();

    let score = games
        .iter()
        .filter_map(|g| {
            if g.rounds
                .iter()
                .any(|r| r.blue > NUM_BLUE || r.green > NUM_GREEN || r.red > NUM_RED)
            {
                tracing::error!("Discarding game {}", &g.id);
                None
            } else {
                Some(g.id)
            }
        })
        .sum();

    Ok(score)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, game)(input)
}

pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag_no_case("Game "), u32, tag_no_case(": "))(input)?;
    let (input, rounds) = separated_list0(tag(";"), round)(input)?;
    Ok((input, Game { id, rounds }))
}

pub fn round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list0(
        tag(","),
        separated_pair(
            preceded(space0, u32),
            space0,
            delimited(space0, alpha1, space0),
        ),
    )(input)?;

    let round = cubes
        .into_iter()
        .fold(Round::default(), |mut round, (count, color)| {
            if color == "red" {
                round.red = count;
            } else if color == "blue" {
                round.blue = count;
            } else if color == "green" {
                round.green = count;
            }
            round
        });
    Ok((input, round))
}

#[derive(Debug, Clone, Default)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Round {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
