const INPUT: &'static str = include_str!("../input");

fn main() {
    let mut lines = INPUT.lines();
    let time_line = lines.next().unwrap();
    let dist_line = lines.next().unwrap();

    let times = time_line
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let distances = dist_line
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut win_counts: Vec<usize> = Vec::new();

    for i in 0..times.len() {
        let time = times[i];
        let dist = distances[i];

        let mut win_count = 0;

        for j in 0..time {
            let movement_per_step = j;
            let time_to_move = time - j;
            let distance_moved = time_to_move * movement_per_step;

            if distance_moved > dist {
                win_count += 1;
            }
        }

        win_counts.push(win_count);
    }

    let result = win_counts
        .into_iter()
        .reduce(|acc, curr| acc * curr)
        .unwrap();

    dbg!(result);
}
