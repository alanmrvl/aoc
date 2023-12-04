use serde::Deserialize;

#[derive(Deserialize, Debug)]
enum MonkeyOperation {
    Add,
    Multiply,
}

#[derive(Deserialize, Debug)]
enum MonkeyOperand {
    Old,
    Const(usize),
}

#[derive(Deserialize, Debug)]
struct MonkeyExpression {
    op: MonkeyOperation,
    first: MonkeyOperand,
    second: MonkeyOperand,
}

#[derive(Deserialize, Debug)]
struct Monkey {
    starting_items: Vec<usize>,
    expression: MonkeyExpression, 
    divisible_by: usize,
    truth_monkey: usize,
    lie_monkey: usize,
}

fn main() {
    println!("Hello, world!");
}
