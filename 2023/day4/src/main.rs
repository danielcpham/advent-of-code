use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0, space0};
use nom::character::streaming::line_ending;
use nom::multi::separated_list0;

use nom::combinator::{map, map_res};
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    winning_nums: HashSet<u16>,
    nums: Vec<u16>,
}

impl Card {
    fn new(winning_nums: Vec<u16>, nums: Vec<u16>) -> Self {
        Self {
            winning_nums: HashSet::from_iter(winning_nums.into_iter()),
            nums,
        }
    }
}

fn get_score(card: &Card) -> usize {
    card.nums.iter().fold(0, |acc, num| {
        match (card.winning_nums.contains(num), acc > 0) {
            (true, true) => acc * 2,
            (true, false) => 1,
            (_, _) => acc,
        }
    })
}

fn get_num_matches(card: &Card) -> usize {
    card.nums
        .iter()
        .filter(|num| card.winning_nums.contains(num))
        .count()
}

fn parse_nums(input: &str) -> IResult<&str, Vec<u16>> {
    separated_list0(
        multispace0,
        map_res(digit1, |num| u16::from_str_radix(num, 10)),
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(
        preceded(
            tuple((tag("Card"), space0, digit1, tag(":"), space0)),
            separated_pair(
                parse_nums,
                delimited(multispace0, tag("|"), multispace0),
                parse_nums,
            ),
        ),
        |(winning_nums, nums)| Card::new(winning_nums, nums),
    )(input)
}

fn part1(input: &str) {
    let (_, cards) = separated_list0(line_ending, parse_card)(input).unwrap();
    dbg!(cards.iter().map(get_score).sum::<usize>());
}

fn part2(input: &str) {
    let (_, cards) = separated_list0(line_ending, parse_card)(input).unwrap();
    let mut card_counts: HashMap<usize, usize> =
        cards.iter().enumerate().map(|(i, _)| (i, 1)).collect();

    for (i, card) in cards.iter().enumerate() {
        let num_matches = get_num_matches(&card);
        let current_copies = card_counts.get(&i).unwrap().clone();
        for j in i + 1..=i + num_matches {
            if card_counts.contains_key(&j) {
                card_counts
                    .entry(j)
                    .and_modify(|val| *val += current_copies);
            }
        }
    }
    dbg!(card_counts.into_values().sum::<usize>());
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("../test.txt");
        // let (_, output) = separated_list0(line_ending, parse_card)(input).unwrap();
        // dbg!(output.iter().map(get_score).collect::<Vec<usize>>());
        part2(input);
    }
}
