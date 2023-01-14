fn main() {
    let grid: Vec<Vec<u8>> = std::fs::read_to_string("input")
        .unwrap()
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .map(|x| {
            x.trim()
                .chars()
                .map(|y| y.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid.first().unwrap().len();

    println!("height {}, width {}", height, width);

    let mut visible: Vec<Vec<bool>> = vec![vec![false; width]; height];

    // dbg!(&grid);
    // dbg!(&visible);

    // from left
    for y in 0..height {
        let mut max = grid[y][0];
        visible[y][0] = true;

        for x in 1..width {
            let curr = grid[y][x];

            if curr > max {
                max = curr;
                visible[y][x] = true;
            }
        }
    }

    // from right
    for y in 0..height {
        let mut max = grid[y][width - 1];
        visible[y][width - 1] = true;

        for x in (1..width).rev() {
            let curr = grid[y][x];

            if curr > max {
                max = curr;
                visible[y][x] = true;
            }
        }
    }

    // from top
    for x in 1..width {
        let mut max = grid[0][x];
        visible[0][x] = true;

        for y in 0..height {
            let curr = grid[y][x];

            if curr > max {
                max = curr;
                visible[y][x] = true;
            }
        }
    }

    // from bottom
    for x in 1..width {
        let mut max = grid[height - 1][x];
        visible[height - 1][x] = true;

        for y in (0..height).rev() {
            let curr = grid[y][x];

            if curr > max {
                max = curr;
                visible[y][x] = true;
            }
        }
    }

    let counts: Vec<usize> = visible
        .into_iter()
        .map(|x| x.into_iter().filter(|y| *y).count())
        .collect();

    let result: usize = counts.iter().sum();

    println!("result: {}", result);
}
