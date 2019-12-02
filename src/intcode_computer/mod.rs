enum OpCodes {
    ADD,
    MULTIPLY,
    HALT,
}

impl OpCodes {
    fn new(t: i32) -> Self {
        match t {
            1 => OpCodes::ADD,
            2 => OpCodes::MULTIPLY,
            99 => OpCodes::HALT,
            _ => panic!("INVALID OP CODE"),
        }
    }

    fn param_count(&self) -> usize {
        match self {
            OpCodes::ADD => 4,
            OpCodes::MULTIPLY => 4,
            OpCodes::HALT => 1,
        }
    }

    fn process(&self, program: &mut Vec<i32>, instruction_ptr: usize) -> Action {
        match self {
            OpCodes::ADD => OpCodes::process_add(program, instruction_ptr),
            OpCodes::MULTIPLY => OpCodes::process_multiply(program, instruction_ptr),
            OpCodes::HALT => Action::HALT,
        }
    }

    fn process_add(instruction: &mut Vec<i32>, instruction_ptr: usize) -> Action {
        let location = instruction[instruction_ptr + 3] as usize;
        let op1 = instruction[instruction[instruction_ptr + 1] as usize];
        let op2 = instruction[instruction[instruction_ptr + 2] as usize];
        instruction[location] = op1 + op2;
        Action::INCREMENT(OpCodes::ADD.param_count())
    }

    fn process_multiply(instruction: &mut Vec<i32>, instruction_ptr: usize) -> Action {
        let location = instruction[instruction_ptr + 3] as usize;
        let op1 = instruction[instruction[instruction_ptr + 1] as usize];
        let op2 = instruction[instruction[instruction_ptr + 2] as usize];
        instruction[location] = op1 * op2;
        Action::INCREMENT(OpCodes::MULTIPLY.param_count())
    }
}

enum Action {
    INCREMENT(usize),
    HALT,
}

pub fn process(program: &mut Vec<i32>) {
    let mut ins_ptr = 0;
    loop {
        let op_code = OpCodes::new(program[ins_ptr]);
        match op_code.process(program, ins_ptr) {
            Action::HALT => break,
            Action::INCREMENT(inc) => ins_ptr = ins_ptr + inc,
        }
    }
}
