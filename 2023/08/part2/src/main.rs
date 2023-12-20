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

    let mut count: usize = 0;
    let mut cycle_lens: HashMap<usize, usize> = HashMap::new();
    let mut currs: Vec<&str> = elements
        .keys()
        .filter(|el| el.ends_with('A'))
        .map(|el| *el)
        .collect();

    let len = currs.len();

    'outer: loop {
        for instruction in &instructions {
            count += 1;

            for (i, curr) in currs.iter_mut().enumerate() {
                let (ref left, ref right) = elements.get(curr).unwrap();

                let next = match instruction {
                    LeftRight::Left => left,
                    LeftRight::Right => right,
                };

                *curr = next;

                if curr.ends_with('Z') {
                    cycle_lens.insert(i, count);

                    if cycle_lens.len() == len {
                        break 'outer;
                    }
                }
            }
        }
    }

    let cycle_lens: Vec<usize> = cycle_lens.into_values().collect();

    let res = lcm(cycle_lens);

    dbg!(res);
}

fn lcm(xs: Vec<usize>) -> usize {
    let mut ans: usize = 1;

    for x in xs {
        ans = (x * ans) / gcd(x, ans);
    }

    return ans;
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}
