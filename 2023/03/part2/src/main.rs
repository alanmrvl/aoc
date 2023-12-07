use std::collections::HashSet;

const INPUT: &'static str = include_str!("../input");

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct EngineNumber {
    value: usize,
    num_digits: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct EngineSymbol {
    value: char,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum EngineComponentType {
    Number(EngineNumber),
    Symbol(EngineSymbol),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct EngineComponent {
    comp_type: EngineComponentType,
    position: Coordinates,
}

fn main() {
    let height = INPUT.lines().count();
    let width = INPUT.lines().nth(1).unwrap().len();

    let mut comps: Vec<EngineComponent> = vec![];

    let mut in_num = false;
    let mut num_start_idx = 0;

    INPUT.lines().enumerate().for_each(|(row, line)| {
        for (i, ch) in line.chars().enumerate() {
            // if we ended the last row on a number, slap it in
            if i == 0 && in_num {
                let last_line = INPUT.lines().nth(row - 1).unwrap();
                let comp = num_comp(&last_line, num_start_idx, width, row - 1);
                comps.push(comp);
                in_num = false;
            }

            // if it's a digit, we are in the middle of a number
            if ch.is_digit(10) {
                if !in_num {
                    num_start_idx = i;
                }
                in_num = true;
                continue;
            }

            // if we were previously in a number, enter in that number
            if in_num {
                let comp = num_comp(&line, num_start_idx, i, row);
                comps.push(comp);
                in_num = false;
            }

            // if we're on a symbol, enter in that symbol
            if ch != '.' {
                let comp = EngineComponent {
                    comp_type: EngineComponentType::Symbol(EngineSymbol { value: ch }),
                    position: Coordinates { x: i, y: row },
                };
                comps.push(comp);
            }
        }
    });

    // create grid of squares that are "adjacent" to a symbol
    let mut grid: Vec<Vec<Vec<EngineComponent>>> = vec![vec![vec![]; width]; height];

    for comp in comps.iter() {
        if let EngineComponentType::Number(num) = &comp.comp_type {
            let y = comp.position.y;
            for x in (comp.position.x)..(comp.position.x + num.num_digits) {
                grid[y][x].push(comp.clone());
            }
        }
    }

    let mut total = 0;

    for comp in comps.iter() {
        if let EngineComponentType::Symbol(EngineSymbol { value: '*' }) = comp.comp_type {
            let mut adjacent_nums = HashSet::new();

            for x in (comp.position.x - 1)..=(comp.position.x + 1) {
                for y in (comp.position.y - 1)..=(comp.position.y + 1) {
                    for &num in grid[y][x].iter() {
                        adjacent_nums.insert(num);
                    }
                }
            }

            // if exactly two adjacent nums, multiple their value
            if adjacent_nums.len() == 2 {
                let val = adjacent_nums.iter().fold(1, |acc, nn| {
                    let curr = match nn.comp_type {
                        EngineComponentType::Number(n) => n.value,
                        _ => panic!(),
                    };
                    return acc * curr;
                });

                total += val;
            }
        }
    }

    println!("{total}");
}

fn num_comp(line: &str, x_start: usize, x_end: usize, y: usize) -> EngineComponent {
    let num = line.get(x_start..x_end).unwrap();
    let comp = EngineComponent {
        comp_type: EngineComponentType::Number(EngineNumber {
            value: num.parse().unwrap(),
            num_digits: num.len(),
        }),
        position: Coordinates { x: x_start, y: y },
    };
    return comp;
}
