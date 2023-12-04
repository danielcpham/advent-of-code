use std::ops::RangeInclusive;

use itertools::Itertools;

fn check_self_contained_range(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    (range1.contains(range2.start()) && range1.contains(range2.end()))
        || (range2.contains(range1.start()) && range2.contains(range1.end()))
}

fn has_overlap(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    (range1.contains(range2.start()) || range1.contains(range2.end()))
        || (range2.contains(range1.start()) || range2.contains(range1.end()))
}

fn parse_range(sec: &str) -> RangeInclusive<u32> {
    sec.split("-")
        .map(|n| n.parse().expect(&format!("Could not parse {}", sec)))
        .collect_tuple::<(u32, u32)>()
        .map(|(start, end)| start..=end)
        .expect("Range needs start and end")
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(parse_range)
                .collect_tuple::<(_, _)>()
                .expect("Need 2 ranges")
        })
        .filter(|(range1, range2)| check_self_contained_range(range1, range2))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(parse_range)
                .collect_tuple::<(_, _)>()
                .expect("Need 2 ranges")
        })
        .filter(|(range1, range2)| has_overlap(range1, range2))
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_range() {
        use crate::parse_range;
        let range = "2-4";
        println!("{:?}", parse_range(range));
    }

    #[test]
    fn test_part1() {
        use super::part1;
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    }
}
