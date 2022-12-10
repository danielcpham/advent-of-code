use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("calories.txt").unwrap();
    let re = Regex::new(r"\s{2,}").unwrap();
    let elf_entries = re.split(&contents).filter(|s| !s.is_empty());
    let mut calories_by_elf = elf_entries
        .map(|x| x.split("\n"))
        .map(|entries| {
            entries
                .filter_map(|entry_str| entry_str.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    calories_by_elf.sort_by(|a, b| b.cmp(a));
    println!("Top Elf Calories: {:?}", calories_by_elf.first().unwrap());
    println!(
        "Top 3 Total: {:?}",
        calories_by_elf[0..3].into_iter().sum::<i32>()
    );
}
