use std::cmp;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../input");

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl Game {
    fn power(&self) -> usize {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        self.rounds.iter().for_each(|r| {
            max_red = cmp::max(max_red, r.red);
            max_green = cmp::max(max_green, r.green);
            max_blue = cmp::max(max_blue, r.blue);
        });

        return max_red * max_green * max_blue;
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, body) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(':'))
            .ok_or(ParseGameError)?;

        let game_id = header.parse::<usize>().map_err(|_| ParseGameError)?;

        let rounds: Vec<Round> = body
            .split(';')
            .map(|s| s.trim())
            .map(|s| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                s.split(',').map(|s| s.trim()).for_each(|c| {
                    let (count, color) = c.split_once(' ').unwrap();

                    let color = color.trim();
                    let count = count.trim().parse::<usize>().unwrap();

                    match color {
                        "red" => red += count,
                        "green" => green += count,
                        "blue" => blue += count,
                        _ => panic!(),
                    }
                });

                return Round {
                    red: red,
                    green: green,
                    blue: blue,
                };
            })
            .collect();

        let res = Game {
            id: game_id,
            rounds: rounds,
        };

        return Ok(res);
    }
}

fn main() {
    let res: usize = INPUT
        .lines()
        .map(|s| s.parse::<Game>().unwrap())
        .map(|game| game.power())
        .sum();

    println!("{:?}", res);
}
