pub use day3::get_priority;
use std::{collections::HashSet, str::Lines};

fn part1(lines: Lines) -> i16 {
	lines
		.map(|line| {
			let (first, second) = line.split_at(line.len() / 2);
			second
				.bytes()
				.filter(|b| b.is_ascii() && first.contains(*b as char))
				.map(get_priority)
				.collect::<HashSet<i16>>()
				.iter()
				.sum::<i16>()
		})
		.sum::<i16>()
}

fn part2(lines: Lines) -> i16 {
	0
}

fn main() {
	let cases = include_str!("../cases.txt").lines();
}

#[cfg(test)]
mod tests {

	#[test]
	fn test_case1() {
		use crate::part1;
		let input = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
		let res = part1(input.lines());
		assert_eq!(res, 157);
	}

}