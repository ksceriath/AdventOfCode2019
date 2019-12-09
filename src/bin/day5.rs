use advent_of_code_2019::intcode_computer;
use advent_of_code_2019::intcode_computer::IO;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

fn main() {
    let mut input = fs::read_to_string("resources/day5.input").unwrap();
    input.pop();
    let instructions = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    #[derive(Clone)]
    struct InOutput {
        output: Rc<RefCell<i32>>,
    };

    impl IO for InOutput {
        fn read(&mut self) -> i32 {
            5
        }

        fn write(&mut self, data: i32) {
            self.output.replace(data);
        }
    }

    let output = Rc::new(RefCell::new(-1));
    let out_c = output.clone();

    let io: Option<Box<dyn IO>> = Some(Box::new(InOutput { output }));

    let mut clone = intcode_computer::Program::new(instructions.clone(), io);
    intcode_computer::process(&mut clone);

    println!("Output: {:?}", out_c);
}
