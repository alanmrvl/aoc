const INPUT: &'static str = include_str!("../input");

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

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
    for i in 0..line.len() {
        if let Some(num) = substr_starts_with_digit(line, i) {
            fst = num;
            sec = num;
            break;
        }
    }

    // get second digit
    for i in (0..line.len()).rev() {
        if let Some(num) = substr_ends_with_digit(line, i) {
            sec = num;
            break;
        }
    }

    let res = fst * 10 + sec;

    // println!("{line} {fst} {sec} {res}\n");

    return res;
}

fn substr_starts_with_digit(line: &str, idx: usize) -> Option<u32> {
    if let Some(num) = digit_char_at_pos(line, idx) {
        return Some(num);
    }

    let substr = line.get(idx..)?;

    for (i, digit) in DIGITS.iter().enumerate() {
        if substr.starts_with(digit) {
            return Some((i + 1) as u32);
        }
    }

    return None;
}

fn substr_ends_with_digit(line: &str, idx: usize) -> Option<u32> {
    if let Some(num) = digit_char_at_pos(line, idx) {
        return Some(num);
    }

    let substr = line.get(..idx + 1)?;

    for (i, digit) in DIGITS.iter().enumerate() {
        if substr.ends_with(digit) {
            return Some((i + 1) as u32);
        }
    }

    return None;
}

fn digit_char_at_pos(line: &str, idx: usize) -> Option<u32> {
    let ch = line.chars().nth(idx)?;
    return ch.to_digit(10);
}
