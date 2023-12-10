use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, one_of},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, (instruction_list, nodes)) =
        parse_input(input).map_err(|e| AocError::NomParseError(e.to_string()))?;
    let current_nodes = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<&&str>>();

    let z_steps: Vec<_> = current_nodes
        .iter()
        .map(|n| {
            let mut instructions: VecDeque<_> = instruction_list.clone();
            let mut count = 0u64;
            let mut current_node = *n;
            while !current_node.ends_with('Z') {
                let (left_node, right_node) = nodes.get(current_node).expect("Invalid Node");
                match instructions.front().expect("No node at front") {
                    Instruction::L => current_node = left_node,
                    Instruction::R => current_node = right_node,
                }
                count += 1;
                instructions.rotate_left(1);
            }
            return count;
        })
        .collect();
    dbg!(&z_steps);
    let c = z_steps.iter().fold(1u64, |lcm, z_val| {
        // calculate the least common multiple

        //first calculate the greatest common factor
        let mut greatest_common_factor = *z_val.max(&lcm);
        let mut remainder = *z_val.min(&lcm);
        while remainder != 0 {
            let temp = greatest_common_factor;
            greatest_common_factor = remainder;
            remainder = temp % remainder;
        }

        //least common multiple
        lcm * (*z_val / greatest_common_factor)
    });
    assert_ne!(c, u64::MAX);
    Ok(c)
}

pub fn parse_input(
    input: &str,
) -> IResult<&str, (VecDeque<Instruction>, HashMap<&str, (&str, &str)>)> {
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
    separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(input)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum Instruction {
    L,
    R,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input3.txt");
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
