use std::iter::zip;

use nom::{
    bytes::complete::take_till,
    character::complete::{newline, space1, u32},
    multi::separated_list1,
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, races) = parse_input(input).map_err(|e| AocError::NomParseError(e.to_string()))?;

    let number_of_ways_to_win = races
        .iter()
        .fold(1, |number_of_ways, (max_time, min_distance)| {
            let number_of_ways_for_race = calculate_distance_range_size(*max_time, *min_distance);
            number_of_ways * number_of_ways_for_race
        });

    Ok(number_of_ways_to_win)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, _) = take_till(nom::AsChar::is_dec_digit)(input)?;
    let (input, times) = separated_list1(space1, u32)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = take_till(nom::AsChar::is_dec_digit)(input)?;
    let (input, distances) = separated_list1(space1, u32)(input)?;
    let zipped_values = zip(times, distances).collect();
    Ok((input, zipped_values))
}

pub fn calculate_distance_range_size(t_max: u32, min_distance: u32) -> u32 {
    let quadratic_formula_sqrt = f64::sqrt(f64::from(t_max.pow(2) - 4 * min_distance));
    let min_time =
        f64::ceil(((t_max as f64 - quadratic_formula_sqrt) / 2.0) + 10.0 * f64::EPSILON) as u32;
    let max_time =
        f64::floor(((t_max as f64 + quadratic_formula_sqrt) / 2.0) - 10.0 * f64::EPSILON) as u32;
    max_time - min_time + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
