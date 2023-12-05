const INPUT: &'static str = include_str!("../input");

fn main() {
    let res: u32 = INPUT
        .lines()
        .map(get_num)
        .sum();

    println!("{res}");
}

fn get_num(line: &str) -> u32 {
    let mut fst = 0;
    let mut sec = 0;

    // get first digit
    for char in line.chars() {
        if char.is_digit(10) {
            fst = char.to_digit(10).unwrap();
            sec = fst;
            break;
        }
    }

    // get second digit
    for char in line.chars().rev() {
        if char.is_digit(10) {
            sec = char.to_digit(10).unwrap();
            break;
        }
    }

    return fst * 10 + sec;
}
