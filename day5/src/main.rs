use std::fs;

use itertools::Itertools;

#[derive(Clone)]
struct Stack {
    crates: Vec<char>,
}

fn show_stacks(stacks: &[Stack]) {
    let mut res = String::new();
    for (idx, stack) in stacks.iter().enumerate() {
        res.push_str(format!("Stack {}, {:?}\n", idx, stack.crates).as_str());
    }
    println!("{}", res);

}

struct MoveInstruction {
    num: usize,
    start: usize,
    end: usize,
}

fn part1(content: String) -> Option<String> {
    // Split into the starting stacks and movement instruction strings
    let (left, instructions_str) = content.split_once("\n\n").unwrap();
    // Split into the starting stacks and platform numbers
    let (stacks_str, platforms) = left.rsplit_once('\n').unwrap();
    let num_stacks = platforms
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut stacks = vec![Stack { crates: Vec::new() }; num_stacks];
    for line in String::from(stacks_str).lines() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second_char = chunk.nth(1).unwrap();
            if second_char.is_alphabetic() {
                stacks[idx].crates.insert(0, second_char)
            }
        }
    }

    show_stacks(&stacks);
    
    let mut moves: Vec<MoveInstruction> = Vec::new();
    for line in String::from(instructions_str).lines() {
        let rest = line.strip_prefix("move ")?;
        let (amount, rest) = rest.split_once(" from ")?;
        let (from, to) = rest.split_once(" to ")?;

        moves.push(MoveInstruction {
            num: amount.parse::<usize>().ok()?,
            start: from.parse::<usize>().ok()?,
            end: to.parse::<usize>().ok()?,
        });
    }

    for inst in moves {
        println!("Move {} from {} to {}", inst.num, inst.start, inst.end);
        let pop_idx = stacks[inst.start - 1].crates.len() - inst.num;
        // to do part2, just remove the .rev()
        // let mut new_stack = stacks[inst.start - 1].crates.drain(pop_idx..).collect();
        let mut new_stack = stacks[inst.start - 1].crates.drain(pop_idx..).rev().collect();
        stacks[inst.end - 1].crates.append(&mut new_stack);
        show_stacks(&stacks);
    }

    let mut answer = String::new();

    for stack in stacks {
        answer.push(*stack.crates.last()?);
    }
    Some(answer)
}

fn main() {
    let result = part1(include_str!("../input.txt").into()).unwrap();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.into());
        println!("{:?}", &result);
    }
}
