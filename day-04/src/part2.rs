use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, games) =
        crate::part1::parse_input(input).map_err(|e| AocError::ParseError(e.to_string()))?;

    let mut game_counts = vec![1usize; games.len()];
    game_counts[0] = 1;

    for (game_number, (winning_numbers, available_numbers)) in games.iter().enumerate() {
        let current_count = game_counts[game_number];

        if current_count == 0 {
            break;
        }

        let num_matches = winning_numbers.iter().fold(0usize, |num_matches, num| {
            if available_numbers.contains_key(num) {
                num_matches + 1
            } else {
                num_matches
            }
        });

        for i in 1..=num_matches {
            if let Some(count) = game_counts.get_mut(i + game_number) {
                *count += current_count;
            }
        }
    }

    let num_games = game_counts.iter().sum();
    Ok(num_games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(30, process(input)?);
        Ok(())
    }
}
