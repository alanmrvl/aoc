use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("input").unwrap();
    let lines = std::io::BufReader::new(file).lines();

    let mut coords = vec![(0, 0); 10];

    let mut visited: HashMap<(isize, isize), usize> = HashMap::new();

    visited.insert(coords[9], 1);

    for line in lines {
        let line = line.unwrap();
        let (direction, count) = line.split_once(" ").unwrap();
        let count = count.parse().unwrap();

        println!("{line:?}");

        for _ in 0..count {
            let hmove = match direction {
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => unreachable!("no other moves"),
            };

            coords[0] = update_position(coords[0], hmove);

            for i in 0..9 {
                let tmove = calculate_tail_move(&coords[i], &coords[i + 1]);

                coords[i + 1] = update_position(coords[i + 1], tmove);
            }

            if let Some(tvisited) = visited.get_mut(&coords[9]) {
                *tvisited += 1;
            } else {
                visited.insert(coords[9], 1);
            }

            // print_grid(&coords);
            // println!();

            // for i in 0..10 {
            //     println!("{i:?}: {:?}",coords[i]);
            // }
        }
    }

    println!();
    println!("Result: {}", visited.len());
}

fn update_position(coord: (isize, isize), pmove: (isize, isize)) -> (isize, isize) {
    return (coord.0 + pmove.0, coord.1 + pmove.1);
}

fn calculate_tail_move(hcoord: &(isize, isize), tcoord: &(isize, isize)) -> (isize, isize) {
    let xdiff = hcoord.0 - tcoord.0;
    let ydiff = hcoord.1 - tcoord.1;

    if xdiff.abs() <= 1 && ydiff.abs() <= 1 {
        return (0, 0);
    }

    let x = if xdiff.is_positive() {
        xdiff - 1
    } else {
        xdiff + 1
    };

    let y = if ydiff.is_positive() {
        ydiff - 1
    } else {
        ydiff + 1
    };

    if xdiff.abs() > 1 && ydiff.abs() > 1 {
        return (x, y);
    } else if xdiff.abs() > 1 {
        return (x, ydiff);
    } else {
        return (xdiff, y);
    }
}

fn print_grid(coords: &Vec<(isize, isize)>) {
    let mut locs = vec![vec![".".to_string(); 26]; 26];

    for (i, (x, y)) in coords.iter().enumerate() {
        let x = *x as usize;
        let y = *y as usize;

        locs[y][x] = if i == 0 {
            "H".to_string()
        } else if locs[y][x] != "." {
            continue;
        } else if i == coords.len() - 1 {
            "T".to_string()
        } else {
            i.to_string()
        }
    }

    for loc in locs.iter().rev() {
        println!("{}", loc.join(""));
    }
}
