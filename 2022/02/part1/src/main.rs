use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum GameMove {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = BufReader::new(file).lines();

    let mut score = 0;

    for line in lines {
        if let Ok(ip) = line {
            let mut moves = ip.split(' ');
            let p1_move: GameMove = match moves.next().unwrap() {
                "A" => GameMove::Rock,
                "B" => GameMove::Paper,
                "C" => GameMove::Scissors,
                _ => panic!("invalid input"),
            };
            let p2_move: GameMove = match moves.next().unwrap() {
                "X" => GameMove::Rock,
                "Y" => GameMove::Paper,
                "Z" => GameMove::Scissors,
                _ => panic!("invalid input"),
            };
            score = score + score_p2_points(&p1_move, &p2_move);
        }
    }

    println!("{}", score);
}

fn score_p2_points(p1: &GameMove, p2: &GameMove) -> i32 {
    let victory_points = score_p2_victory_points(&p1, &p2);

    return victory_points
        + match p2 {
            GameMove::Rock => 1,
            GameMove::Paper => 2,
            GameMove::Scissors => 3,
        };
}

fn score_p2_victory_points(p1: &GameMove, p2: &GameMove) -> i32 {
    if p1 == p2 {
        return 3;
    }

    return match (p1, p2) {
        (GameMove::Rock, GameMove::Paper) => 6,
        (GameMove::Rock, GameMove::Scissors) => 0,
        (GameMove::Paper, GameMove::Rock) => 0,
        (GameMove::Paper, GameMove::Scissors) => 6,
        (GameMove::Scissors, GameMove::Rock) => 6,
        (GameMove::Scissors, GameMove::Paper) => 0,
        (_, _) => panic!("This should never happen"),
    };
}
