use nom::sequence::terminated;
use nom::Finish;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let starting_bag = (12, 13, 14);
    let games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line).finish().map(|(_, game)| game))
        .collect::<Result<Vec<Game>, _>>()
        .unwrap();

    games
        .into_iter()
        .filter_map(|game| game.is_possible(starting_bag).then_some(game.id))
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line).finish().map(|(_, game)| game))
        .collect::<Result<Vec<Game>, _>>()
        .unwrap();
    dbg!(games.into_iter().map(|game| game.calculate_power()).sum::<usize>())
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse::<usize>())(input)
}

fn parse_records(input: &str) -> IResult<&str, Vec<Vec<(usize, &str)>>> {
    separated_list1(
        tag("; "),
        separated_list1(
            tag(", "),
            alt((
                tuple((parse_number, preceded(tag(" "), tag("blue")))),
                tuple((parse_number, preceded(tag(" "), tag("green")))),
                tuple((parse_number, preceded(tag(" "), tag("red")))),
            )),
        ),
    )(input)
}
fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = terminated(preceded(tag("Game "), parse_number), tag(": "))(input)?;
    let (input, records) = parse_records(input)?;
    let parsed_records = records
        .into_iter()
        .map(|rec| {
            rec.into_iter()
                .fold((0, 0, 0), |(red, green, blue), (num, color)| match color {
                    "red" => (red + num, green, blue),
                    "green" => (red, green + num, blue),
                    "blue" => (red, green, blue + num),
                    _ => (red, green, blue),
                })
        })
        .collect();
    Ok((
        input,
        Game {
            id,
            rounds: parsed_records,
        },
    ))
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<(usize, usize, usize)>,
}

impl Game {
    fn is_possible(&self, starting_bag: (usize, usize, usize)) -> bool {
        for round in self.rounds.iter().cloned() {
            if starting_bag.0 < round.0 || starting_bag.1 < round.1 || starting_bag.2 < round.2 {
                println!("ID: {}: {:?} < {:?}", self.id, starting_bag, round);
                return false;
            }
        }
        true
    }

    fn calculate_power(&self) -> usize {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in self.rounds.iter().cloned() {
            if round.0 > max_red {
                max_red = round.0;
            }
            if round.1 > max_green {
                max_green = round.1;
            }
            if round.2 > max_blue {
                max_blue = round.2;
            }
        }
        max_red * max_green * max_blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        part1(EXAMPLE);
    }

    #[test]
    fn test_part2() {
        part2(EXAMPLE);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")]
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")]
    fn read_game_record(#[case] input: &str) {
        parse_game(input);
    }
}
