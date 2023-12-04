use itertools::Itertools;

fn get_priority(b: u8) -> u32 {
    (match b {
        b'a'..=b'z' => (b - b'a') + 1,
        _ => (b - b'A') + 1 + 26,
    }) as u32
}
fn get_sack_priority(line: &str) -> Option<u32> {
    let (first, second) = line.as_bytes().split_at(line.len() / 2);
    first
        .iter()
        .filter(|b| b.is_ascii_alphabetic())
        .find(|b| second.contains(b))
        .and_then(|b| Some(get_priority(*b) as u32))
}

fn part1(sacks: &str) -> u32 {
    sacks.lines().filter_map(get_sack_priority).sum::<u32>()
}

fn get_badge_priorities_sum(sack_list: &str) -> u32 {
    sack_list
        .lines()
        .tuples() //
        .filter_map(|(sack1, sack2, sack3)| {
            sack1
                .find(|x| sack2.contains(x) && sack3.contains(x))
                .and_then(|i| sack1.as_bytes().get(i))
        }).map(|b| get_priority(*b))
        .sum::<u32>()
}


fn part2(groups: &str) -> u32 {
    get_badge_priorities_sum(groups)
}

fn main() {
    let part1_res = part1(include_str!("../cases.txt"));
    let res = part2(include_str!("../cases.txt"));
    println!("{:?}", part1_res);
    println!("{:?}", res);
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
        let res = part1(input);
        assert_eq!(res, 157);
    }
	#[test]
    fn test_get_badge() {
        use super::get_badge_priorities_sum;
        let input = include_str!("../day3b_test.txt");
        get_badge_priorities_sum(input);
    }
}
