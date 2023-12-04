use std::collections::HashMap;

use itertools::{intersperse, Itertools};

use nom::bytes::complete::take_until;
use nom::character::complete::{alpha1, u32};
use nom::sequence::tuple;
use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::newline,
    multi::separated_list1,
    sequence::preceded,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(CdPath<'a>),
    Ls(Vec<DirContent<'a>>),
}

#[derive(Debug)]
enum CdPath<'a> {
    Root,
    Up,
    Down(&'a str),
}

const TOTAL_DISK_SPACE: u32 = 70000000;

#[derive(Debug)]
enum DirContent<'a> {
    File { name: &'a str, size: u32 },
    Dir { name: &'a str },
}
fn parse_file(input: &str) -> IResult<&str, DirContent> {
    let (input, (size, _, name)) = tuple((u32, tag(" "), take_until("\n")))(input)?;
    Ok((input, DirContent::File { size, name }))
}

fn parse_dir(input: &str) -> IResult<&str, DirContent> {
    let (input, name) = preceded(tag("dir "), alpha1)(input)?;
    Ok((input, DirContent::Dir { name }))
}

fn parse_ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tuple((tag("$ ls"), newline))(input)?;
    let (input, files) = separated_list1(newline, alt((parse_file, parse_dir)))(input)?;

    Ok((input, Operation::Ls(files)))
}

fn parse_cd(input: &str) -> IResult<&str, Operation> {
    let (remaining, path) = preceded(tag("$ cd "), take_till(|c| c == '\n'))(input)?;
    match path {
        ".." => Ok((remaining, Operation::Cd(CdPath::Up))),
        "/" => Ok((remaining, Operation::Cd(CdPath::Root))),
        _ => Ok((remaining, Operation::Cd(CdPath::Down(path)))),
    }
}

fn parser(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmds) = separated_list1(newline, alt((parse_cd, parse_ls)))(input)?;
    Ok((input, cmds))
}

fn part1(input: &str) -> u32 {
    let (_, ops) = parser(input).unwrap();
    println!("{:#?}", ops);

    let mut workdir = Vec::new();
    let mut file_map: HashMap<String, Vec<(&str, u32)>> = HashMap::new();
    for op in ops {
        match op {
            Operation::Cd(CdPath::Up) => {
                workdir.pop();
                if workdir.is_empty() {
                    workdir.push("root");
                }
            }
            Operation::Cd(CdPath::Root) => {
                workdir.clear();
                workdir.push("root");
            }
            Operation::Cd(CdPath::Down(path)) => {
                workdir.push(path);
            }
            Operation::Ls(contents) => {
                for content in contents {
                    if let DirContent::File { name, size } = content {
                        file_map
                            .entry(intersperse(workdir.iter().cloned(), "/").collect())
                            .and_modify(|vec| vec.push((name, size)))
                            .or_insert(vec![(name, size)]);
                    }
                }
            }
        }
    }
    dbg!(&file_map);
    let mut size_map = HashMap::new();
    for (dir_path, files) in file_map.iter() {
        let sum: u32 = files.iter().map(|(_, size)| size).sum();
        let dirs = dir_path.split('/').collect::<Vec<&str>>();
        dbg!(&dirs);
        for i in 0..dirs.len() {
            size_map
                .entry(intersperse(dirs[0..=i].iter().cloned(), "/").collect::<String>())
                .and_modify(|val| *val += sum)
                .or_insert(sum);
        }
    }
    dbg!(&size_map);
    // size_map
    //     .iter()
    //     .filter(|(_, &size)| size <= 100000)
    //     .map(|(_, size)| size)
    // .sum::<u32>()
    let total_space = size_map.remove("root").unwrap();
    size_map
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .filter(|(_, size)| total_space - *size <= 40000000)
        .map(|(_, &size)| size).collect::<Vec<u32>>()[0]
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 95437);
        let test_str = "$ cd /
$ ls
dir a
$ cd a
$ ls
dir a
2 a.txt
$ cd a
$ ls
99999 a.txt
";
        assert_eq!(part1(test_str), 99999);
    }
}
