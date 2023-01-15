use std::cmp;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let grid: Vec<Vec<u8>> = std::fs::read_to_string("input")
        .unwrap()
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .map(|x| {
            x.trim()
                .chars()
                .map(|y| y.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid.first().unwrap().len();

    let grid = Arc::new(grid);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    println!("height {}, width {}", height, width);

    for y in 0..height {
        for x in 0..width {
            let tgrid = Arc::clone(&grid);
            let tcounter = Arc::clone(&counter);

            let handle = thread::spawn(move || {
                let el = tgrid[y][x];
                let mut final_score = 0;

                // left
                let mut curr_score = 0;
                for i in (0..x).rev() {
                    let curr = tgrid[y][i];

                    if curr >= el {
                        curr_score = x - i;
                        break;
                    } else if i == 0 {
                        curr_score = x;
                        break;
                    }
                }
                final_score = calc_score(final_score, curr_score);
                // println!("({y}, {x}) L :: {curr_score}");

                // right
                let mut curr_score = 0;
                for i in x + 1..width {
                    let curr = tgrid[y][i];

                    if curr >= el {
                        curr_score = i - x;
                        break;
                    } else if i == width - 1 {
                        curr_score = width - 1 - x;
                        break;
                    }
                }
                final_score = calc_score(final_score, curr_score);
                // println!("({y}, {x}) R :: {curr_score}");

                // up
                let mut curr_score = 0;
                for i in (0..y).rev() {
                    let curr = tgrid[i][x];

                    if curr >= el {
                        curr_score = y - i;
                        break;
                    } else if i == 0 {
                        curr_score = y;
                        break;
                    }
                }
                final_score = calc_score(final_score, curr_score);
                // println!("({y}, {x}) L :: {curr_score}");

                // down
                let mut curr_score = 0;
                for i in y + 1..height {
                    let curr = tgrid[i][x];

                    if curr >= el {
                        curr_score = i - y;
                        break;
                    } else if i == height - 1 {
                        curr_score = height - 1 - y;
                        break;
                    }
                }
                final_score = calc_score(final_score, curr_score);
                // println!("({y}, {x}) R :: {curr_score}");

                let mut max = tcounter.lock().unwrap();
                *max = cmp::max(*max, final_score);
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *counter.lock().unwrap();

    println!("Result: {result:?}");
}

fn calc_score(final_score: usize, curr_score: usize) -> usize {
    return if final_score == 0 {
        curr_score
    } else if curr_score == 0 {
        final_score
    } else {
        final_score * curr_score
    };
}
