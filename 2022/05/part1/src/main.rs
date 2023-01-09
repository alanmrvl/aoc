use std::io::BufRead;

fn main() {
    let path = std::path::Path::new("input");
    let file = std::fs::File::open(path).unwrap();
    let lines = std::io::BufReader::new(file).lines().skip(10);

    let mut pos = vec![
        vec!['J', 'H', 'P', 'M', 'S', 'F', 'N', 'V'],
        vec!['S', 'R', 'L', 'M', 'J', 'D', 'Q'],
        vec!['N', 'Q', 'D', 'H', 'C', 'S', 'W', 'B'],
        vec!['R', 'S', 'C', 'L'],
        vec!['M', 'V', 'T', 'P', 'F', 'B'],
        vec!['T', 'R', 'Q', 'N', 'C'],
        vec!['G', 'V', 'R'],
        vec!['C', 'Z', 'S', 'P', 'D', 'L', 'R'],
        vec!['D', 'S', 'J', 'V', 'G', 'P', 'B', 'F']
    ];

    for line in lines {
        let line = line.unwrap();
        
        let words: Vec<&str> = line.split(" ").collect();

        let mut move_count = words[1].parse::<u32>().unwrap();
        let from_loc: usize = words[3].parse::<usize>().unwrap() - 1;
        let to_loc:usize = words[5].parse::<usize>().unwrap() - 1;

        while move_count > 0 {
            let item = pos[from_loc].pop().unwrap();
            pos[to_loc].push(item);
            move_count = move_count - 1;
        }
    }

    let mut last_pos: Vec<char> = vec![];
    
    for p in pos {
        if let Some(last) = p.last() {
            last_pos.push(*last);
        }
    }

    let result: String = last_pos.iter().collect();

    println!("{}", result);
}
