use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, newline, one_of},
    combinator::{map_res, not, opt},
    multi::{fold_many1, separated_list0},
    sequence::delimited,
    IResult, Parser,
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
        delimited(opt(not(alt((parse_digit_number, parse_digit_word)))), one_of("0123456789"), one_of("abcd")),
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
            res.1.unwrap_or_else(|| first_digit),
        ),
    ))
}

fn parse_digit_word(input: &str) -> IResult<&str, u32> {
    alt((
        parse_zero,
        parse_one,
        parse_two,
        parse_three,
        parse_four,
        parse_five,
        parse_six,
        parse_seven,
        parse_eight,
        parse_nine,
    ))(input)
}

fn parse_one(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("one"), |_| 1)(input)
}

fn parse_two(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("two"), |_| 2)(input)
}
fn parse_three(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("three"), |_| 3)(input)
}
fn parse_four(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("four"), |_| 4)(input)
}
fn parse_five(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("five"), |_| 5)(input)
}
fn parse_six(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("six"), |_| 6)(input)
}
fn parse_seven(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("seven"), |_| 7)(input)
}
fn parse_eight(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("eight"), |_| 8)(input)
}
fn parse_nine(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("nine"), |_| 9)(input)
}
fn parse_zero(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(tag("zero"), |_| 0)(input)
}

fn parse_digit_number(input: &str) -> IResult<&str, u32> {
    nom::combinator::map(one_of("123456789"), |d| d.to_digit(10).unwrap())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
