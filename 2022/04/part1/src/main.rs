use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::Split;

fn main() {
    let path = Path::new("input");
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut score = 0;

    for line_result in lines {
        let line = line_result.unwrap();
        let assignments: Split<&str> = line.split(",");

        let assignment_ranges: Vec<Vec<i32>> = assignments
            .map(|x| x.split("-").map(|y| y.parse::<i32>().unwrap()).collect())
            .collect();

        let elf1 = &assignment_ranges[0];
        let elf2 = &assignment_ranges[1];

        if subsumes_other(&elf1, &elf2) || subsumes_other(&elf2, &elf1) {
            // println!("{:?} v {:?}", elf1, elf2);
            score = score + 1;
        }
    }

    println!("{}", score);
}

fn subsumes_other(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    return first[0] <= second[0] && first[1] >= second[1];
}
