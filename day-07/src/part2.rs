use std::collections::HashMap;

use nom::{
    character::complete::{newline, one_of, space1, u32},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, mut hands) = parse_input(input).map_err(|e| AocError::NomParseError(e.to_string()))?;

    hands.sort();
    let score = hands.iter().enumerate().fold(0, |score, (rank, hand)| {
        score + (((rank as u32) + 1) * hand.bid)
    });

    Ok(score)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(newline, hand)(input)
}

pub fn hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = many1(card)(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = u32(input)?;
    let mut counts = cards.iter().fold(HashMap::with_capacity(5), |mut map, v| {
        map.entry(v).and_modify(|c| *c += 1).or_insert(1u8);
        map
    });

    let hand_kind = calculate_hand_kind(&mut counts);

    let hand = Hand {
        cards,
        bid,
        kind: hand_kind,
    };

    Ok((input, hand))
}

pub fn calculate_hand_kind(counts: &mut HashMap<&u8, u8>) -> Kind {
    if let Some(joker_counts) = counts.remove(&1) {
        match counts.values().len() {
            0 | 1 => Kind::FiveOfAKind,
            2 => {
                let max_count = counts.values().max().unwrap();
                if max_count + joker_counts == 4 {
                    Kind::FourOfAKind
                } else {
                    Kind::FullHouse
                }
            }
            3 => Kind::ThreeOfAKind,
            4 => Kind::OnePair,
            _ => panic!("Impossible count value reached with a joker"),
        }
    } else {
        match counts.values().len() {
            5 => Kind::HighCard,
            4 => Kind::OnePair,
            3 => counts
                .values()
                .find_map(|count| {
                    if *count == 2 {
                        Some(Kind::TwoPair)
                    } else {
                        None
                    }
                })
                .unwrap_or(Kind::ThreeOfAKind),
            2 => counts
                .values()
                .find_map(|count| {
                    if *count == 2 {
                        Some(Kind::FullHouse)
                    } else if *count == 1 {
                        Some(Kind::FourOfAKind)
                    } else {
                        None
                    }
                })
                .expect("key count 2 with no joker had invalid combination"),
            1 => Kind::FiveOfAKind,
            _ => panic!("Invalid combination of cards with no joker"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<u8>,
    bid: u32,
    kind: Kind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            ord => ord,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn card(input: &str) -> IResult<&str, u8> {
    map(one_of("AKQJT98765432"), |c| match c {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card character"),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(5905, process(input)?);
        Ok(())
    }
}
