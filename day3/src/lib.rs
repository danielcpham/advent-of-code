use std::collections::HashSet;
pub fn get_priority_char(c: char) -> i16 {
    get_priority(c as u8)
}
pub fn get_priority(b: u8) -> i16 {
    if b >= b'a' {
        (b - b'a') as i16 + 1
    } else {
        (b - b'A') as i16 + 27
    }
}
pub fn get_badge(rucksacks: &str) -> Option<char> {
    let badge = rucksacks
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii() && !c.is_whitespace())
                .collect::<HashSet<char>>()
        })
        .reduce(|acc, sack| acc.intersection(&sack).cloned().collect())
        .unwrap();

    match badge.len() {
        1 => badge.iter().next().copied(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_priority() {
        use super::get_priority_char;
        assert_eq!(get_priority_char('p'), 16);
        assert_eq!(get_priority_char('L'), 38);
        assert_eq!(get_priority_char('P'), 42);
        assert_eq!(get_priority_char('v'), 22);
        assert_eq!(get_priority_char('t'), 20);
        assert_eq!(get_priority_char('s'), 19);
    }

    #[test]
    fn test_get_badge() {
        use super::get_badge;
        let group1 = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
    ";
        let group2 = "
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    ";

        assert_eq!(Some('r'), get_badge(group1));
        assert_eq!(Some('Z'), get_badge(group2));
    }
}
