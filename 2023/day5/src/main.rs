use std::{cmp::min, collections::HashMap};
use rayon::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, newline, space1},
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

// struct ReferenceMap {
//     source_type: &'static str,
//     map: HashMap<usize, usize>
// }
fn parse_digits(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list0(space1, map_res(digit1, |num| usize::from_str_radix(num, 10)))(input)
}
#[derive(Debug)]
struct Range {
    source_start: usize,
    dest_start: usize,
    range: usize
}

impl From<Vec<usize>> for Range {
    fn from(value: Vec<usize>) -> Self {
        let slice: &[usize] = value.as_slice();
        Self {
            dest_start: slice[0],
            source_start: slice[1],
            range: slice[2]

        }
    }
}

impl Range {
    fn get_destination(&self, source: usize) -> Option<usize> {
        let source_range = self.source_start..self.source_start+self.range;
        match source_range.contains(&source) {
            false => None,
            true => Some((source - self.source_start) + self.dest_start)
        }
    }
}

#[derive(Debug)]
struct Category {
    source: String,
    dest: String,
    ranges: Vec<Range>
}


impl Category {
    fn new(source: &str, dest: &str, values: Vec<Vec<usize>>) -> Self {
        Self {
            source: String::from(source),
            dest: String::from(dest),
            ranges: values.into_iter().map(Range::from).collect()
        }
    }

    fn convert(&self, loc: usize) -> usize {
        self.ranges.iter()
        .filter_map(|range| range.get_destination(loc))
        .collect::<Vec<usize>>().first().copied().unwrap_or(loc)
    }
}

fn parse_map(input: &str) -> IResult<&str, Category> {
    let (input, (source, dest)) = terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        tuple((tag(" map:"), newline)),
    )(input)?;
    let (input, values) = separated_list1(
        newline,
            separated_list1(
            space1,
            map_res(digit1, |num_str| usize::from_str_radix(num_str, 10)),
        ),
    )(input)?;
    // dbg!(&values);
    Ok((
        input,
        Category::new(source, dest, values)
    ))
}

fn parse_almanac(input: &str) -> IResult<&str, (Vec<usize>, HashMap<String, Category>)> {
    let (input, seeds) = delimited(
        tuple((tag("seeds:"), multispace0)),
        parse_digits,
        multispace0,
    )(input)?;
    let (input, cats) = separated_list1(tag("\n\n"), parse_map)(input)?;

    let cat_maps = cats.into_iter().map(|map| (map.source.clone(), map)).collect();
    // dbg!(&cat_maps);
    Ok((input, (seeds, cat_maps)))
}

fn part1(input: &str) {
    let (_, (seeds, cat_maps)) = parse_almanac(input).unwrap();
    let mut min_val = usize::MAX;
    for seed in seeds {
        let mut source_map = "seed";
        let mut src_val = seed;

        while let Some(cat_map) = cat_maps.get(source_map) {
            src_val = dbg!(cat_map.convert(src_val));
            source_map = cat_map.dest.as_str();
        }
        min_val = min(min_val, dbg!(src_val));
    }
    dbg!(min_val);




}

fn part2(input: &str) {
    let (_, (seeds, cat_maps)) = parse_almanac(input).unwrap();
    let mut seeds_to_check: Vec<usize> = Vec::new();
    for seed_range in seeds.chunks(2) {
        // dbg!(seed_range);
        seeds_to_check.extend(seed_range[0]..seed_range[0]+seed_range[1]);
    }
    // dbg!(&seeds_to_check);
    let locations = seeds_to_check.par_iter().map(|seed| {
        let mut source_map = "seed";
        let mut src_val = seed.clone();

        while let Some(cat_map) = cat_maps.get(source_map) {
            src_val = cat_map.convert(src_val);
            source_map = cat_map.dest.as_str();
        }
        dbg!(src_val.to_owned())
    });

    dbg!(locations.min());
    // for seed in seeds_to_check {
    //     let mut source_map = "seed";
    //     let mut src_val = seed;

    //     while let Some(cat_map) = cat_maps.get(source_map) {
    //         src_val = cat_map.convert(src_val);
    //         source_map = cat_map.dest.as_str();
    //     }
    //     min_val = min(min_val, dbg!(src_val));
    // }
    // dbg!(min_val);
}

fn main() {
    let input = include_str!("../input.txt");
    // part1(input);
    part2(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        part1(input);
    }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("../test.txt");
    //     part2(input);
    // }
}
