use advent_of_code_2019::intcode_computer;
use std::fs;

fn main() {
    let mut input = fs::read_to_string("resources/day5.input").unwrap();
    input.pop();
    let instructions = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let output = Vec::new();
    let program = intcode_computer::Program::new(instructions, vec![5], output);

    let mut clone = program.clone();
    intcode_computer::process(&mut clone);

    println!("Output: {:?}", clone.output);
}
