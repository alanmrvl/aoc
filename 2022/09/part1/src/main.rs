use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("input").unwrap();
    let lines = std::io::BufReader::new(file).lines();

    let mut hcoord = (0, 0);
    let mut tcoord = (0, 0);

    let mut visited: HashMap<(isize, isize), usize> = HashMap::new();

    visited.insert(tcoord, 1);

    for line in lines {
        let line = line.unwrap();
        let (direction, count) = line.split_once(" ").unwrap(); 
        let count = count.parse().unwrap();

        for _ in 0..count {
            let hmove = match direction {
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => unreachable!("no other moves"),
            };

            update_position(&mut hcoord, hmove);

            let tmove = calculate_tail_move(&hcoord, &tcoord);

            update_position(&mut tcoord, tmove);

            if let Some(tvisited) = visited.get_mut(&tcoord) {
               *tvisited +=1; 
            } else {
                visited.insert(tcoord, 1);
            }

            // println!("{line:?}");
            // println!("H {hcoord:?} T {tcoord:?}");
        }
    }

    println!("{}", visited.len());
}

fn update_position(coord: &mut (isize, isize), pmove: (isize, isize)) {
    coord.0 += pmove.0;
    coord.1 += pmove.1;
}

fn calculate_tail_move(hcoord: &(isize, isize), tcoord: &(isize, isize)) -> (isize, isize) {
    let xdiff = hcoord.0 - tcoord.0;
    let ydiff = hcoord.1 - tcoord.1;

    if xdiff.abs() <= 1 && ydiff.abs() <= 1 {
        return (0, 0);
    }

    if xdiff.abs() > 1 {
        let x = if xdiff.is_positive() { xdiff - 1 } else { xdiff + 1 };
        return (x, ydiff);
    } else {
        let y = if ydiff.is_positive() { ydiff - 1 } else { ydiff + 1 };
        return (xdiff, y);
    }
}
