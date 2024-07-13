use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
}

#[derive(Debug)]
struct SchematicNumber {
    row: usize,
    col: usize,
    value: String,
}

impl SchematicNumber {
    fn get_surroundings(&self, num_rows: usize, num_cols: usize) -> HashSet<(usize, usize)> {
        println!(
            "get_surroundings: ({}, {}) ({})",
            self.row,
            self.col,
            self.value.len()
        );
        let row_range = match self.row == 0 {
            true => self.row..=(self.row + 1),
            false => (self.row - 1)..=(self.row + 1),
        };
        let col_range = match self.col == 0 {
            true => self.col..=(self.col + self.value.len()),
            false => (self.col - 1)..=(self.col + self.value.len()),
        };
        row_range
            .into_iter()
            .flat_map(|row| std::iter::repeat(row).zip(col_range.clone()))
            .filter(|(row, col)| {
                !(*row == self.row && *col >= self.col && *col < self.col + self.value.len())
            })
            .collect::<HashSet<(usize, usize)>>()
    }
}

fn part1(input: &str) {
    let mut part_candidates = Vec::new();
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    let mut symbol_map = HashMap::new();

    let rows = input.lines();
    let num_rows = rows.count();
    let num_cols = input.lines().next().unwrap().len();
    for (row, line) in input.lines().enumerate() {
        let mut part_str = String::new();
        let mut starting_col = 0;
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if part_str.is_empty() {
                    starting_col = col;
                }
                part_str.push(ch);
                symbol_map.insert((row, col), ch);
            } else {
                if !part_str.is_empty() {
                    part_candidates.push(SchematicNumber {
                        row,
                        col: starting_col,
                        value: part_str,
                    });
                    part_str = String::new();
                }
                if ch != '.' {
                    symbol_map.insert((row, col), ch);
                    symbols.insert((row, col));
                }
            }
        }
    }
    dbg!(symbol_map);

    dbg!(&part_candidates.iter().map(|num| &num
        .get_surroundings(num_rows, num_cols)
        .into_iter()
        .map(|(row, col)| &symbol_map.entry((row, col)))));

    let res: Option<usize> = dbg!(part_candidates
        .iter()
        // .map(|num| format!(
        //     "{}, {}, {}: has surroundings? {}",
        //     num.row,
        //     num.col,
        //     num.value,
        //     !(symbols.is_disjoint(&num.get_surroundings(num_cols, num_rows)))
        // ))
        .filter(|num| !(symbols.is_disjoint(&num.get_surroundings(num_cols, num_rows))))
        .filter_map(|num| num.value.parse::<usize>().ok())
        .sum::<usize>()
        .into());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        part1(EXAMPLE);
    }
}
