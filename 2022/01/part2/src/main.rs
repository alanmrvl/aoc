use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = BufReader::new(file).lines();

    let mut elf_max: Vec<i32> = vec![0, 0, 0];
    let mut elf_curr = 0;

    for line in lines {
        if let Ok(ip) = line {
            if let Ok(line_num) = ip.parse::<i32>() {
                elf_curr = elf_curr + line_num
            } else {
                append_max(&mut elf_max, elf_curr);
                elf_curr = 0;
            }
        }
    }

    append_max(&mut elf_max, elf_curr);

    println!("{}", elf_max.iter().sum::<i32>());
}

fn append_max(elf_max: &mut Vec<i32>, curr: i32) {
    let mut curr_i: i32 = curr;

    for i in 0..elf_max.len() {
        let max_i = elf_max[i];

        if curr_i > max_i {
            elf_max[i] = curr_i;
            curr_i = max_i;
        }
    }
}
