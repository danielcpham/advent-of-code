use std::{collections::HashSet, cmp};

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input));
}

#[derive(Clone)]
struct Forest {
    width: usize,
    height: usize,
    trees: Vec<u32>,
}

impl Forest {
    fn new(input: &str) -> Self {
        let (first, _) = input.split_once('\n').unwrap();
        let width = first.chars().count();
        let height = input.lines().clone().count();

        let trees = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        Forest {
            width,
            height,
            trees,
        }
    }

    fn select_row(&self, row: usize) -> Vec<u32> {
        assert!(self.height > row);
        let idxs = (self.width * row)..(self.width * (row + 1));
        self.trees[idxs].to_vec()
    }

    fn select_col(&self, col: usize) -> Vec<u32> {
        assert!(self.width > col);
        self.trees
            .iter()
            .copied()
            .skip(col)
            .step_by(self.width)
            .collect::<Vec<u32>>()
    }

    fn scan_row(&self, row: usize) -> Vec<(usize, usize)> {
        let mut visible_trees = Vec::new();
        let mut max_tree_height: u32 = 0;
        for (col, tree) in self.select_row(row).into_iter().enumerate() {
            if col == 0 || tree > max_tree_height {
                visible_trees.push((row, col));
                max_tree_height = tree;
            }
        }

        max_tree_height = 0;
        for (col, tree) in self.select_row(row).into_iter().enumerate().rev() {
            if col == self.width - 1 || tree > max_tree_height {
                if visible_trees.contains(&(row, col)) {
                    return visible_trees;
                }
                visible_trees.push((row, col));
                max_tree_height = tree;
            }
        }
        visible_trees
    }

    fn scan_col(&self, col: usize) -> Vec<(usize, usize)> {
        let mut visible_trees = Vec::new();
        let mut max_tree_height: u32 = 0;
        for (row, tree) in self.select_col(col).into_iter().enumerate() {
            if row == 0 || tree > max_tree_height {
                visible_trees.push((row, col));
                max_tree_height = tree;
            }
        }

        max_tree_height = 0;
        for (row, tree) in self.select_col(col).into_iter().enumerate().rev() {
            if row == self.width - 1 || tree > max_tree_height {
                if visible_trees.contains(&(row, col)) {
                    return visible_trees;
                }
                visible_trees.push((row, col));
                max_tree_height = tree;
            }
        }
        visible_trees
    }
}

fn parse_forest(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect()
}
fn part1(input: &str) -> usize {
    //trees are visible if they are strictly increasing in the viewing
    // direction.
    //
    let forest = parse_forest(input);
    let num_cols = forest.len();
    let num_rows = forest[0].len();
    let visible_trees: Vec<Vec<bool>> = dbg!(forest
        .iter()
        .enumerate()
        .map(|(row, trees)| {
            let mut max_height = 0;
            trees
                .iter()
                .enumerate()
                .map(|(col, tree)| {
                    if row == 0 || row == num_rows - 1 || col == 0 || col == num_cols - 1 {
                        true
                    } else if max_height < *tree {
                        max_height = *tree;
                        true
                    } else {
                        false
                    }
                })
                .collect()
        })
        .collect());
    for col in 0..num_cols {
        let max_height = 0;
        for row in 0..num_rows {
            forest[row][col]
        }
    }

    // let forest = Forest::new(input);
    // let mut visible_trees = Vec::new();
    // visible_trees.extend((0..forest.width).flat_map(|row| forest.scan_row(row)));
    // visible_trees.extend((0..forest.height).flat_map(|col| forest.scan_col(col)));
    // dbg!(&visible_trees);

    // // dbg!((0..width*height).step_by(width).collect::<Vec<usize>>())
    // let index_vectors = (0..width).map(|i| {
    //    (0..width*height).skip(i).step_by(width).collect::<Vec<usize>>()
    // }).collect::<Vec<Vec<usize>>>();
    //
    // let vals = &index_vectors.iter().map(|i_vec| i_vec.iter().map(|idx| forest[idx]).collect()).collect();
    // let from_iter: HashSet<(usize, usize)> = HashSet::from_iter(visible_trees);
    // from_iter.extend(visible_trees);
    // println!("{:?}", &from_iter);
    // from_iter.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let res = part1(EXAMPLE);
        assert_eq!(res, 21);
    }
}
