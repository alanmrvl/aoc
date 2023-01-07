use std::cmp::max;
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

    let mut elf_max = 0;
    let mut elf_curr = 0;

    for line in lines {
        if let Ok(ip) = line {
            if let Ok(line_num) = ip.parse::<i32>() {
                elf_curr = elf_curr + line_num
            } else {
                elf_max = max(elf_max, elf_curr);
                elf_curr = 0;
            }
        }
    }
    elf_max = max(elf_max, elf_curr);
    println!("{}", elf_max);
}
