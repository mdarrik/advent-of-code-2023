use std::ops::Range;

use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, newline, space1, u32},
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, almanac) = parse_input(input).map_err(|e| AocError::NomParseError(e.to_string()))?;

    let min_destination = almanac.seeds.iter().map(|seed| {
        almanac.maps.iter().fold(*seed, |current_val, map| {
            let Some(matching_map) = map.iter().find(|m| m.start.contains(&current_val)) else {
                return current_val;
            };
            let offset = current_val - matching_map.start.start;
            matching_map.destination.start + offset
        })
    }).min().ok_or(AocError::EmptyResult)?;
    assert_ne!(min_destination, u32::MAX);
    dbg!(u32::MAX);
   Ok(min_destination)
}

pub fn parse_input(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, u32)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, maps) = separated_list1(permutation((newline, opt(newline))), parse_map_list)(input)?;
    Ok((input, Almanac {seeds, maps}))
}

pub fn parse_map_list(input: &str) -> IResult<&str, Vec<AlmanacMap>> {
    let (input, _) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
    let (input, _) = preceded(space1, tag("map:"))(input)?;
    let (input, _) = newline(input)?;

    let (input, map) = separated_list1(newline, parse_map)(input)?;

    Ok((input, map))
}

pub fn parse_map(input: &str) -> IResult<&str, AlmanacMap> {
    let (input, destination_start) = u32(input)?;
    let (input, _) = space1(input)?;
    let (input, source_start) = u32(input)?;
    let (input, _) = space1(input)?;
    let (input, map_size) = u32(input)?;

    let almanac_map = AlmanacMap {start: source_start..(source_start + map_size), destination: destination_start..(destination_start + map_size) };

    Ok((input, almanac_map)) 
}

#[derive(Debug, Default)]
pub struct Almanac {
    pub seeds: Vec<u32>,
    pub maps: Vec<Vec<AlmanacMap>>,
}

#[derive(Debug, Default)]
pub struct AlmanacMap {
    pub destination: Range<u32>,
    pub start: Range<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(35, process(input)?);
        Ok(())
    }
}
