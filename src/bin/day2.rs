use advent_of_code_2019::intcode_computer;
use std::fs;

fn main() {
    let mut input = fs::read_to_string("resources/day2.input").unwrap();
    input.pop();
    let program = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    const PROGRAM_OUTPUT: i32 = 19690720;

    'outer: for i in 0..100 {
        for j in 0..100 {
            let mut clone = program.clone();
            clone[1] = i;
            clone[2] = j;

            intcode_computer::process(&mut clone);
            if clone[0] == PROGRAM_OUTPUT {
                let code = 100 * i + j;
                println!("code : {}", code);
                break 'outer;
            }
        }
    }
}
