use std::str::FromStr;

#[derive(Debug)]
enum Cmd {
    Noop,
    Addx(isize),
}

#[derive(Debug, PartialEq, Eq)]
struct CmdParseError;

impl FromStr for Cmd {
    type Err = CmdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Cmd::Noop);
        }

        let (_, offset) = s.split_once(" ").unwrap();

        let offset = offset.parse::<isize>().unwrap();

        return Ok(Cmd::Addx(offset));
    }
}

fn main() {
    // let cmds: Vec<isize> = Some(0)
    //     .into_iter()
    //     .chain(
    //         include_str!("../input")
    //             .lines()
    //             .map(|x| x.parse::<Cmd>().unwrap())
    //             .map(|x| match x {
    //                 Cmd::Noop => 0,
    //                 Cmd::Addx(num) => num,
    //             }),
    //     )
    //     .collect();

    let cmds: Vec<isize> = include_str!("../input")
        .lines()
        .map(|x| x.parse::<Cmd>().unwrap())
        .flat_map(|x| match x {
            Cmd::Noop => vec![0],
            Cmd::Addx(num) => vec![0, num],
        })
        .collect();

    // println!("{cmds:?}");

    let mut res: Vec<isize> = Vec::with_capacity(cmds.len());

    res.push(1);

    for cmd in cmds.iter() {
        let last = res.last().unwrap_or(&1);
        res.push(last + cmd);
    }

    // println!();
    // println!("{res:?}");

    let result: isize = res
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| {
            let cycle = i as isize + 1;
            let product = cycle * x;
            // println!("{cycle:03} * {x:04} = {product:06}");
            return product;
        })
        .sum();

    println!("Result: {result:?}");

    let mut crt = [["."; 40]; 6];

    // let res: Vec<_> = res.iter().skip(19).collect();

    for i in 0..6 {
        for j in 0..40 {
            let ind = (i * 40) + j;
            let sprint_position = res[ind];

            let ind = j as isize;

            // println!("i {i}; j {j}; ind {ind}; sprint_position {}, {sprint_position}, {}", sprint_position - 1, sprint_position +1);

            if ind >= sprint_position - 1 && ind <= sprint_position + 1 {
                crt[i][j] = "#";
                // println!("#");
            } else {
                // println!(".");
            }

            // println!();
        }
    }

    for i in 0..crt.len() {
        println!("{:?}", crt[i].join(""));
    }
}
