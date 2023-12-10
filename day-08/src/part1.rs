use std::collections::{VecDeque, HashMap};

use nom::{character::complete::{one_of, newline, alpha1}, multi::{fold_many1, separated_list1}, IResult, sequence::{separated_pair, delimited}, bytes::complete::tag};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, (mut instruction_list, nodes)) = parse_input(input).map_err(|e| AocError::NomParseError(e.to_string()))?;

    let mut current_node = "AAA";

    let mut count = 0;

    while current_node != "ZZZ" {
        let (left_node, right_node) = nodes.get(current_node).expect("Invalid Node");
        match instruction_list.front().expect("No node at front") {
            Instruction::L => {current_node = left_node},
            Instruction::R => {current_node = right_node}
        }
        count += 1;
        instruction_list.rotate_left(1);
    }

    Ok(count)

}

pub fn parse_input(input: &str) -> IResult<&str, (VecDeque<Instruction>, HashMap<&str, (&str, &str)>)> {
    let (input, instruction_list) = instructions(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, nodes) = separated_list1(newline, node)(input)?;
    let nodes = nodes.into_iter().collect::<HashMap<&str, (&str, &str)>>();

    Ok((input, (instruction_list, nodes)))
}

pub fn instructions(input: &str) -> IResult<&str, VecDeque<Instruction>> {
    let (input, acc) = fold_many1(
        one_of("LR"),
        || VecDeque::new(),
        |mut v, c| {
            if c == 'L' {
                v.push_back(Instruction::L);
            } else {
                v.push_back(Instruction::R);
            }
            v
        },
    )(input)?;
    Ok((input, acc))
}

pub fn node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
   separated_pair(alpha1, tag(" = "), delimited(tag("("), separated_pair(alpha1, tag(", "), alpha1), tag(")")))(input)

}

#[repr(usize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    L,
    R,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_input() -> miette::Result<()> {
        let input = include_str!("../test-input1.txt");
        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test]
    fn test_repeating_instructions() -> miette::Result<()> {
        let input = include_str!("../test-input2.txt");
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
