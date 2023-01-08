use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum GameMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
enum GameResult {
    Lose,
    Draw,
    Win,
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
            let p2_result: GameResult = match moves.next().unwrap() {
                "X" => GameResult::Lose,
                "Y" => GameResult::Draw,
                "Z" => GameResult::Win,
                _ => panic!("invalid input"),
            };
            let p2_move: GameMove = get_p2_move(&p1_move, &p2_result);
            // println!("{:?} v {:?} v {:?}", p1_move, p2_result, p2_move);
            score = score + score_p2_points(&p1_move, &p2_move);
        }
    }

    println!("{}", score);
}

fn get_p2_move(p1_move: &GameMove, p2_result: &GameResult) -> GameMove {
    return match (p1_move, p2_result) {
        (GameMove::Rock, GameResult::Lose) => GameMove::Scissors,
        (GameMove::Rock, GameResult::Draw) => GameMove::Rock,
        (GameMove::Rock, GameResult::Win) => GameMove::Paper,
        (GameMove::Paper, GameResult::Lose) => GameMove::Rock,
        (GameMove::Paper, GameResult::Draw) => GameMove::Paper,
        (GameMove::Paper, GameResult::Win) => GameMove::Scissors,
        (GameMove::Scissors, GameResult::Lose) => GameMove::Paper,
        (GameMove::Scissors, GameResult::Draw) => GameMove::Scissors,
        (GameMove::Scissors, GameResult::Win) => GameMove::Rock,
    };
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
