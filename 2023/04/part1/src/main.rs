use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../input");

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    played_numbers: Vec<usize>,
}

impl Card {
    /// Calculates the overlap of winning numbers in points
    fn won(&self) -> usize {
        let winning_set: HashSet<_> = self.winning_numbers.iter().collect();

        let result = self
            .played_numbers
            .iter()
            .filter(|n| winning_set.contains(n))
            .count();

        if result == 0 {
            return 0;
        }

        let points = 2u64.pow(result as u32 - 1) as usize;

        return points;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_name, all_nums) = s.split_once(':').ok_or(ParseCardError)?;

        let id: usize = game_name
            .split(' ')
            .filter(|s| !s.is_empty())
            .last()
            .ok_or(ParseCardError)?
            .parse()
            .map_err(|_| ParseCardError)?;

        let (winning_raw, played_raw) = all_nums.split_once('|').ok_or(ParseCardError)?;

        let winning: Vec<usize> = winning_raw
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.trim().parse().map_err(|_| ParseCardError))
            .collect::<Result<Vec<_>, _>>()?;

        let played: Vec<usize> = played_raw
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.trim().parse().map_err(|_| ParseCardError))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(Card {
            id: id,
            winning_numbers: winning,
            played_numbers: played,
        });
    }
}

fn main() {
    let result: usize = INPUT
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .map(|g| g.won())
        .sum();

    println!("{result}");
}
