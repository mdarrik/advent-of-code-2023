use crate::custom_error::AocError;
use crate::part1::{Round, parse_input};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, games) = parse_input(input).unwrap();

    let power = games.iter().fold(0, |power, game| {
        let ideal_round = game.rounds.iter().fold(Round::default(), |ideal_round, round| {
            Round {
                blue: u32::max(ideal_round.blue, round.blue),
                green: u32::max(ideal_round.green, round.green),
                red: u32::max(ideal_round.red, round.red)
            }
        });
       power + ideal_round.blue * ideal_round.green * ideal_round.red
    });
    Ok(power)
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
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
