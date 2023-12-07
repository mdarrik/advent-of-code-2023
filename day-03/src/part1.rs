use std::collections::HashMap;


use nom::{
    bytes::complete::{take_till},
    character::{
        complete::{newline},
    },
    combinator::{opt},
    AsChar, IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, v) = read_numbers_from_line(input).unwrap();
    dbg!(v.len());
    std::fs::write("vec.txt", format!("{:?}", v)).unwrap();
    let duplicates = v
        .iter()
        .enumerate()
        .filter_map(|(i, l)| {
            let mut duplicate_map = HashMap::new();

            for n in l.iter() {
                if duplicate_map.contains_key(n) {
                    dbg!("duplicate", &i, n, &l);
                    return Some(i);
                }
                duplicate_map.insert(*n, ());
            }
            None
        })
        .collect::<Vec<usize>>();
    dbg!(duplicates);
    todo!("HI");
    let (symbol_list, number_map) = parse_input(input)?;
    std::fs::write("./number_map.txt", format!("{:?}", &number_map)).unwrap();
    // dbg!(&symbol_list.iter().last());
    let mut valid_numbers: HashMap<(usize, u32), _> = HashMap::with_capacity(number_map.len());
    for (symbol_col, symbol_row) in symbol_list {
        for row_modifier in 0..=1 {
            let pos_row_change = symbol_row + row_modifier;
            let neg_row_change = symbol_row.saturating_sub(row_modifier);
            for col_modifier in 0..=1 {
                let pos_col_change = symbol_col + col_modifier;
                let neg_col_change = symbol_col.saturating_sub(col_modifier);
                for (r, c) in [
                    (pos_row_change, pos_col_change),
                    (pos_row_change, neg_col_change),
                    (neg_row_change, pos_col_change),
                    (neg_row_change, neg_col_change),
                ] {
                    if let Some(number) = number_map.get(&(c, r)) {
                        if r == 139 {
                            // dbg!("should have row 139", number);
                        }
                        //cheat here and assume all numbers in a row are unique.
                        valid_numbers.insert((r, *number), *number);
                    }
                }
            }
        }
    }
    std::fs::write("map-dbg", format!("{:?}", &valid_numbers)).unwrap();
    let sum_of_valid_num = valid_numbers.values().sum();
    Ok(sum_of_valid_num)
}

pub fn parse_input(
    input: &str,
) -> Result<(Vec<(usize, usize)>, HashMap<(usize, usize), u32>), AocError> {
    let num_lines = input.lines().count();
    let num_cols = input.lines().next().expect("empty input").len();
    let mut symbol_list = Vec::new();
    let mut number_map = HashMap::with_capacity(num_lines * num_cols / 2);

    let lines = input.lines().enumerate();

    for (row, l) in lines {
        // get all of the numbers in the row
        // let (l, numbers) = nom::combinator::peek(separated_list0(
        //     nom::combinator::not(nom::character::complete::digit1::<&str, nom::error::Error<&str>>),
        //     nom::character::complete::u32,
        // ))(l).unwrap();

        let (_, numbers) = parse_number_line(l).unwrap();

        // .map_err(|e| {
        //     dbg!(&e);
        //     AocError::NomParseError(e.to_string())})?;

        let mut number_index = 0;
        let mut have_hit_number = false;
        l.chars().enumerate().for_each(|(column, c)| {
            if c.is_numeric() {
                let number = numbers[number_index];
                number_map.insert((column, row), number);
                have_hit_number = true;
            } else {
                if have_hit_number {
                    have_hit_number = false;
                    number_index += 1;
                }
                if c != '.' {
                    symbol_list.push((column, row));
                }
            }
        });
    }
    Ok((symbol_list, number_map))
}

pub fn parse_number_line(input: &str) -> IResult<&str, Vec<u32>> {
    let mut digits = vec![];
    let mut consumed_input = input;
    while !consumed_input.is_empty() {
        let (input, _) = take_till(AsChar::is_dec_digit)(consumed_input)?;
        let (input, possible_num) = opt(nom::character::complete::u32)(input)?;
        let (input, _) = opt(newline)(input)?;
        if let Some(num) = possible_num {
            digits.push(num);
        }

        consumed_input = input;
    }

    Ok((input, digits))
}

pub fn parse_number_line_consume(input: &str) -> IResult<&str, Vec<u32>> {
    let mut digits = vec![];
    let mut consumed_input = input;
    while !consumed_input.is_empty() {
        let (input, _) = take_till(AsChar::is_dec_digit)(consumed_input)?;
        let (input, possible_num) = opt(nom::character::complete::u32)(input)?;
        let (input, n) = nom::combinator::peek(opt(newline))(input)?;

        if let Some(num) = possible_num {
            digits.push(num);
        }

        consumed_input = input;
        if n.is_some() {
            break;
        }
    }
    dbg!(consumed_input);
    Ok((consumed_input, digits))
}

pub fn read_numbers_from_line(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let mut lines = vec![];

    for line in input.lines() {
        let (_, l) = parse_number_line(line)?;
        lines.push(l);
    };

    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input1.txt");
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
