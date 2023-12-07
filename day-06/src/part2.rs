use nom::{
    bytes::complete::take_till,
    character::complete::{digit1, newline, space1, u64},
    multi::separated_list1,
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, (time, distance)) =
        parse_input(input).map_err(|e| AocError::NomParseError(e.to_string()))?;
    let number_of_possibilities = calculate_distance_range_size(time, distance);

    Ok(number_of_possibilities)
}

pub fn calculate_distance_range_size(t_max: u64, min_distance: u64) -> u32 {
    let quadratic_formula_sqrt = f64::sqrt((t_max.pow(2) - 4 * min_distance) as f64);
    let min_time =
        f64::ceil(((t_max as f64 - quadratic_formula_sqrt) / 2.0) + 10.0 * f64::EPSILON) as u32;
    let max_time =
        f64::floor(((t_max as f64 + quadratic_formula_sqrt) / 2.0) - 10.0 * f64::EPSILON) as u32;
    max_time - min_time + 1
}

pub fn parse_input(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = take_till(nom::AsChar::is_dec_digit)(input)?;
    let (input, time) = nom::combinator::map_res(separated_list1(space1, digit1), |r| {
        let s = r.join("");
        let (_, s) = u64::<&str, nom::error::Error<&str>>(s.as_str())
            .map_err(|e| AocError::NomParseError(e.to_string()))?;
        Ok::<u64, AocError>(s)
    })(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = take_till(nom::AsChar::is_dec_digit)(input)?;
    let (input, distance) = nom::combinator::map_res(separated_list1(space1, digit1), |r| {
        let s = r.join("");
        let (_, s) = u64::<&str, nom::error::Error<&str>>(s.as_str())
            .map_err(|e| AocError::NomParseError(e.to_string()))?;
        Ok::<u64, AocError>(s)
    })(input)?;
    Ok((input, (time, distance)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
