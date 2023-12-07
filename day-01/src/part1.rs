use nom::{
    character::{
        complete::{alpha0, newline, one_of},
    },
    multi::{fold_many1, separated_list0},
    sequence::delimited, IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let rows = parse(input)?;
    let sum = rows
        .iter()
        .fold(0, |sum, (tens, ones)| sum + (10 * tens) + ones);

    Ok(sum)
}

pub fn parse(input: &str) -> miette::Result<Vec<(u32, u32)>, AocError> {
    let (_, res) = separated_list0(newline, parse_line)(input)
        .map_err(|e| AocError::ParseError(e.to_string()))?;
    Ok(res)
}

fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, res) = fold_many1(
        delimited(alpha0, one_of("0123456789"), alpha0),
        || (None, None),
        |acc, item| {
            if acc.0.is_none() {
                (item.to_digit(10), None)
            } else {
                (acc.0, item.to_digit(10))
            }
        },
    )(input)?;
    let first_digit = res.0.expect("Missing first digit");
    Ok((
        input,
        (
            res.0.expect("Missing first digit"),
            res.1.unwrap_or(first_digit),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
