use std::{cmp::Ordering, fs, str::FromStr};

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let res = match (self, other) {
            (Move::Rock, Move::Scissors) => Ordering::Greater,
            (Move::Paper, Move::Rock) => Ordering::Greater,
            (Move::Scissors, Move::Paper) => Ordering::Greater,
            _ => {
                if self == other {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
        };
        Some(res)
    }
}

#[derive(Debug)]
struct RoundPart1 {
    // A, B, C -> Rock, Paper, Scissors
    opponent: Opponent,
    //X, Y, Z -> Rock, Paper, Scissors
    player: Player,
}

#[derive(Debug)]
struct Opponent(Move);
#[derive(Debug)]
struct Player(Move);


impl FromStr for Opponent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Opponent(Move::Rock)),
            "B" => Ok(Opponent(Move::Paper)),
            "C" => Ok(Opponent(Move::Scissors)),
        _ => Err("Bad Opponent Input".to_string()),
        }
    }
}

fn parse_player_move(s: &str) -> Result<Move, String> {
    match s {
        "X" => Ok(Move::Rock),
        "Y" => Ok(Move::Paper),
        "Z" => Ok(Move::Scissors),
        _ => Err("Bad Player Input".to_string()),

    }
}

impl FromStr for Player {
    type Err = String;
    fn from_str(s: &str) -> Result<Player, String> {
        parse_player_move(&s).and_then(|player| Ok(Player(player)))
    }

}


impl FromStr for RoundPart1 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s.split_whitespace().collect::<Vec<&str>>();
        let opponent = Opponent::from_str(&moves[0]);
        let player_move = Player::from_str(&moves[1]);
        opponent.and_then(|op| {
            player_move.and_then(|p| {
                Ok(RoundPart1 {
                    opponent: op,
                    player: p,
                })
            })
        })
    }
}

fn score_round(round: &RoundPart1) -> i32 {
    let mut score = match round.player.0 {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    if round.opponent.0 == round.player.0 {
        score += 3;
    }
    if round.player.0 > round.opponent.0 {
        score += 6;
    }
    score
}

#[derive(Debug)]
struct RoundPart2 {
    opponent: Opponent,
    player: RoundResult,
}

#[derive(Debug)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl FromStr for RoundResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err("Bad result input".to_string()),
        }
    }
}

impl FromStr for RoundPart2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s.split_whitespace().collect::<Vec<&str>>();
        let opponent = Opponent::from_str(&moves[0]);
        let player_response = RoundResult::from_str(&moves[1]);

        opponent.and_then(|op| {
            player_response.and_then(|p| {
                Ok(RoundPart2 {
                    opponent: op,
                    player: p,
                })
            })
        })
    }

   }

impl RoundPart2 {
     fn get_response(self) -> Move {
        match self.player {
            RoundResult::Draw => self.opponent.0,
            RoundResult::Lose => match self.opponent.0 {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            RoundResult::Win => match self.opponent.0 {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
        }

    }
}

fn part1(input: String) {
    let rounds = input
        .split_terminator("\n")
        .map(|round_str| RoundPart1::from_str(round_str).unwrap());
    //for round in rounds {
    //println!("{:?}, {:?}", &round, score_round(&round));
    //}
    println!("{}", rounds.map(|round| score_round(&round)).sum::<i32>());

}


fn part2(input: String) {
    let rounds = input
        .split_terminator("\n")
        .map(|round_str| RoundPart2::from_str(round_str).unwrap());
    println!("{:?}", rounds.collect::<Vec<RoundPart2>>());
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    //let input = "C X";
    //println!("{:?}", input.split_terminator("\n").collect::<Vec<&str>>());
    //part1(input);
    part2(input);
}
