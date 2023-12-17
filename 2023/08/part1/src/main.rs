use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input");

#[derive(Debug, PartialEq, Eq)]
enum LeftRight {
    Left,
    Right,
}

fn main() {
    let (instructions_raw, elements_raw) = INPUT.split_once("\n\n").unwrap();

    let instructions = instructions_raw
        .chars()
        .map(|c| match c {
            'L' => LeftRight::Left,
            'R' => LeftRight::Right,
            _ => panic!(),
        })
        .collect::<Vec<LeftRight>>();

    // dbg!(&instructions);

    let elements = elements_raw
        .lines()
        .map(|line| {
            let (id, left_right_raw) = line.split_once(" = ").unwrap();

            let (left_raw, right_raw) = left_right_raw.split_once(", ").unwrap();

            let left = left_raw.strip_prefix("(").unwrap();
            let right = right_raw.strip_suffix(")").unwrap();

            return (id, (left, right));
        })
        .collect::<HashMap<_, _>>();

    // dbg!(&elements);

    let mut count = 0;
    let mut curr: &str = "AAA";

    'outer: loop {
        for instruction in &instructions {
            if curr == "ZZZ" {
                break 'outer;
            }

            let (ref left, ref right) = elements.get(&curr).unwrap();

            curr = match instruction {
                LeftRight::Left => left,
                LeftRight::Right => right,
            };

            count += 1;
        }
    }

    dbg!(count);
}
