use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let path: &Path = Path::new("input");
    let file: File = File::open(path).unwrap();
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();

    let mut found: HashSet<char> = HashSet::new();

    let mut score = 0;

    'outer: for line_result in lines {
        found.clear();

        let line: String = line_result.unwrap();

        let mid = line.len() / 2;

        let left: &str = &line[..mid];
        let right: &str = &line[mid..];

        // println!("{}", line);
        // println!("{}::{}", left, right);

        for lchar in left.chars() {
            found.insert(lchar);
        }

        for rchar in right.chars() {
            if found.contains(&rchar) {
                score = score + score_letter(rchar);
                continue 'outer;
            }
        }
    }

    println!("{}", score);
}

fn score_letter(item: char) -> u32 {
    let num = item as u32;
    let a_num = 'a' as u32;
    let a_num_cap = 'A' as u32;

    let offset = if item >= 'a' && item <= 'z' {
        a_num
    } else {
        a_num_cap - 26
    };

    return num - offset + 1;
}
