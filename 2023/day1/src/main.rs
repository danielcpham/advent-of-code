use regex::{Captures, Regex, RegexSet};

fn main() {
    let input: &str = include_str!("../input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> usize {
    let nums = input.lines().map(|line| {
        let chs = line
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<Vec<char>>();

        vec![
            chs.iter().cloned().nth(0),
            chs.iter().cloned().nth(chs.len() - 1),
        ]
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .and_then(|x| String::from_iter(x).parse::<usize>().ok())
        .unwrap()
    });
    dbg!(nums.sum::<usize>())
}

fn part2(input: &str) -> usize {
    let nums = input.lines().map(|line| {
        let replaced_string = replace_string(line);
        let chs = replaced_string
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<Vec<char>>();

        let calib_value = vec![
            chs.iter().cloned().nth(0),
            chs.iter().cloned().nth(chs.len() - 1),
        ];
        calib_value
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .and_then(|x| String::from_iter(x).parse::<usize>().ok())
            .unwrap()
    });
    nums.sum::<usize>()
}

fn replace_string(line: &str) -> String {
    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let p_set = RegexSet::new(patterns).unwrap();
    let set_matches: Vec<_> = p_set.matches(line).into_iter().collect();
    let matched_patterns: Vec<String> = p_set

        .patterns()
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(i, pattern)| match set_matches.contains(&i) {
            true => Some(pattern),
            false => None
        }).collect();

    let mut replaced_string = String::from(line);
    for m in matched_patterns {
        replaced_string = replaced_string.replace(
            m.as_str(),
            match m.as_str() {
                "one" => "o1e",
                "two" => "t2o",
                "three" => "t3e",
                "four" => "f4r",
                "five" => "f5e",
                "six" => "s6x",
                "seven" => "s7n",
                "eight" => "e8t",
                "nine" => "n9e",
                x => x,
            },
        );
    }
    replaced_string
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    #[ignore]
    fn test_part1() {
        part1(EXAMPLE);
    }

    #[test]
    fn test_part2() {
        // part2(EXAMPLE2);
        // part2("9dlvndqbddghpxc");
        assert_eq!(part2("nineight"), 98);
        assert_eq!(part2("nineeight"), 98);
    }

    #[test]
    fn test_regex() {
        assert_eq!(replace_string("nineight"), "n9e8t" );
        assert_eq!(replace_string("nineeight"), "n9ee8t" );
    }
}
