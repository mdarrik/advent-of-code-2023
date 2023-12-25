use std::collections::HashMap;

use ndarray::{Array2, ArrayView};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{fold_many0, many0, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, galaxy_map) =
        parse_input(input).map_err(|e| AocError::NomParseError(e.to_string()))?;

    let empty_cols: Vec<_> = galaxy_map
        .columns()
        .into_iter()
        .enumerate()
        .filter_map(
            |(col_num, col)| {
                if col.sum() == 0 {
                    Some(col_num)
                } else {
                    None
                }
            },
        ).collect();
    let empty_rows: Vec<_> = galaxy_map
        .rows()
        .into_iter()
        .enumerate()
        .filter_map(
            |(row_num, row)| {
                if row.sum() == 0 {
                    Some(row_num)
                } else {
                    None
                }
            },
        ).collect();

    let new_col_length = galaxy_map.ncols() + empty_cols.len();
    let new_row_length = galaxy_map.nrows() + empty_rows.len();

    let expanded_value_map = 


    galaxy_map.indexed_iter().fold(HashMap::new(), |mut map, ((row, col), val)| {
        if val == &1 {
            let new_row = row + empty_rows.iter().filter(|r| row < **r).count();
            let new_col = col + empty_cols.iter().filter(|c| col < **c).count();

            map.
        }
        map
    });

    let expanded_map = Array2::from_shape_fn((new_row_length, new_col_length), |(row,col)| {
       let col_shift = empty_cols.iter().filter(|v| **v < col).count();
       let row_shift = empty_rows.iter().filter(|v| **v < row).count();

       let (Some(shifted_row), Some(shifted_col)) = (row.checked_sub(row_shift), col.checked_sub(col_shift)) else {
        return 0
       };

       if let Some(v) = galaxy_map.get((shifted_row, shifted_col)) {
        *v
       } else {
        0
       }

    });


    
    Ok(0)
}

pub fn parse_input(input: &str) -> IResult<&str, Array2<u8>> {
    let (input, lines) = separated_list1(newline, many0(alt((galaxy, empty_space))))(input)?;
    let array = Array2::from_shape_vec(
        (lines.len(), lines.first().expect("Empty line").len()),
        lines.into_iter().flatten().collect(),
    )
    .expect("Failed to make array");

    Ok((input, array))
}

pub fn galaxy(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("#")(input)?;
    Ok((input, 1))
}

pub fn empty_space(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag(".")(input)?;

    Ok((input, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
