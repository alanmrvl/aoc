use std::cmp;
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
    /// Calculates the overlap of winning numbers
    fn won(&self) -> usize {
        let winning_set: HashSet<_> = self.winning_numbers.iter().collect();

        let result = self
            .played_numbers
            .iter()
            .filter(|n| winning_set.contains(n))
            .count();

        return result;
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
            .map(|n| n.parse().map_err(|_| ParseCardError))
            .collect::<Result<Vec<_>, _>>()?;

        let played: Vec<usize> = played_raw
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().map_err(|_| ParseCardError))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(Card {
            id,
            winning_numbers: winning,
            played_numbers: played,
        });
    }
}

fn main() {
    let mut card_matches: Vec<(usize, usize)> = INPUT
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .map(|g| (g.won(), 1))
        .collect();

    // go through each card, and make copies
    for i in 0..card_matches.len() {
        let (match_count, copy_count) = card_matches[i];

        // for each copy
        for _ in 0..copy_count {
            // add copies equal to the number of matches
            for k in 0..cmp::min(match_count, card_matches.len() - i) {
                let idx = i + k + 1;
                card_matches[idx].1 += 1;
            }
        }
    }

    let result: usize = card_matches
        .into_iter()
        .map(|(_, copy_count)| {
            return copy_count;
        })
        .sum();

    println!("{result}");
}
