use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", part1(input).unwrap());
    println!("{:?}", part2(input).unwrap());
}

fn get_marker(signal: &str, window_size: usize) -> Option<usize> {
    let signal_iter = signal.chars().collect::<Vec<char>>();
    for (start_idx, window) in signal_iter.windows(window_size).enumerate() {
        let char_set: HashSet<&char> = HashSet::from_iter(window.iter());
        if char_set.len() == window_size {
            return Some(start_idx + window_size);
        }
    }
    None

}

fn part1(signal: &str) -> Option<usize> {
    get_marker(signal, 4)
}

fn part2(signal: &str) -> Option<usize> {
    get_marker(signal, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap();
        assert_eq!(result, 7);
    }
}
