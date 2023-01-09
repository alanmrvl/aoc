use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let path: &Path = Path::new("input");
    let file: File = File::open(path).unwrap();
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();

    let mut found: HashMap<char, i32> = HashMap::new();
    let mut found_line: HashSet<char> = HashSet::new();

    let mut score = 0;

    'outer: for (index, line_result) in lines.enumerate() {
        found_line.clear();

        if index % 3 == 0 {
            found.clear();
        }

        let line: String = line_result.unwrap();

        // println!("{}", line);

        for lchar in line.chars() {
            // If we haven't come across this character on the current line,
            // add it to the calculations.
            if !found_line.contains(&lchar) {
                let curr_opt = found.get(&lchar);

                let curr = match curr_opt {
                    Some(curr_exists) => curr_exists,
                    None => &0,
                };

                let new_curr = *curr + 1;

                if new_curr == 3 {
                    score = score + score_letter(lchar);
                    continue 'outer;
                }

                found.insert(lchar, new_curr);
            }

            found_line.insert(lchar);
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

    // println!("{}: {} - {} + 1", item, num, offset);

    return num - offset + 1;
}
