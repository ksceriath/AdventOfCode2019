use log::debug;
use std::thread;

pub trait IO {
    fn read(&mut self) -> i32;
    fn write(&mut self, o: i32);
}

pub struct Program {
    pub instructions: Vec<i32>,
    instruction_ptr: usize,
    io: Option<Box<dyn IO>>,
}

impl Program {
    pub fn new(instructions: Vec<i32>, io: Option<Box<dyn IO>>) -> Self {
        Program {
            instruction_ptr: 0,
            instructions,
            io,
        }
    }

    fn next(&mut self) -> i32 {
        self.instruction_ptr = self.instruction_ptr + 1;
        let ins = self.instructions[self.instruction_ptr - 1];
        debug!("{:?} =>  [ {} : {} ] ", thread::current().name(), self.instruction_ptr - 1, ins);
        ins
    }

    fn set_pointer(&mut self, ptr: usize) {
        self.instruction_ptr = ptr;
    }

    fn at_position(&self, position: usize) -> i32 {
        self.instructions[position]
    }

    fn set_position(&mut self, position: usize, data: i32) {
        self.instructions[position] = data;
    }

    fn set_position_from_input(&mut self, position: usize) {
        debug!("{:?} => SET INPUT AT {}, ", thread::current().name(), position);
        self.instructions[position] = self.from_input();
    }

    fn from_input(&mut self) -> i32 {
        let x = self.io.as_mut().unwrap().read();
        debug!("{:?} => READ INPUT : {:?}", thread::current().name(), x);
        x
    }

    fn to_output(&mut self, data: i32) {
        debug!("{:?} => WRITE OUTPUT : {}", thread::current().name(), data);
        self.io.as_mut().unwrap().write(data);
    }
}

struct Instruction {
    code: OpCodes,
    param_modes: Vec<i32>,
}

impl Instruction {
    fn get_param_modes(n: usize, mut c: i32) -> Vec<i32> {
        let mut p = Vec::new();
        for _ in 0..n {
            p.push(c % 10);
            c = c / 10;
        }
        debug!("{:?} =>  < {:?} > ", thread::current().name(), p);
        p
    }

    fn new(c: i32) -> Self {
        let t = c % 100;
        let code = OpCodes::op_code(t);
        let param_modes = Instruction::get_param_modes(code.param_count(), c / 100);
        Instruction { code, param_modes }
    }

    fn process(&self, program: &mut Program) -> Action {
        if self.code.process(program, &self.param_modes) {
            Action::CONTINUE
        } else {
            Action::HALT
        }
    }
}

enum OpCodes {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl OpCodes {
    fn op_code(t: i32) -> Self {
        match t {
            1 => OpCodes::Add,
            2 => OpCodes::Multiply,
            3 => OpCodes::Input,
            4 => OpCodes::Output,
            5 => OpCodes::JumpIfTrue,
            6 => OpCodes::JumpIfFalse,
            7 => OpCodes::LessThan,
            8 => OpCodes::Equals,
            99 => OpCodes::Halt,
            _ => panic!("INVALID OP CODE"),
        }
    }

    fn param_count(&self) -> usize {
        match self {
            OpCodes::Add => 3,
            OpCodes::Multiply => 3,
            OpCodes::Input => 1,
            OpCodes::Output => 1,
            OpCodes::JumpIfTrue => 2,
            OpCodes::JumpIfFalse => 2,
            OpCodes::LessThan => 3,
            OpCodes::Equals => 3,
            OpCodes::Halt => 0,
        }
    }

    fn process(&self, program: &mut Program, param_modes: &Vec<i32>) -> bool {
        match self {
            OpCodes::Add => OpCodes::process_add(program, param_modes),
            OpCodes::Multiply => OpCodes::process_multiply(program, param_modes),
            OpCodes::Input => OpCodes::process_input(program, param_modes),
            OpCodes::Output => OpCodes::process_output(program, param_modes),
            OpCodes::JumpIfTrue => OpCodes::process_jump_if_true(program, param_modes),
            OpCodes::JumpIfFalse => OpCodes::process_jump_if_false(program, param_modes),
            OpCodes::LessThan => OpCodes::process_less_than(program, param_modes),
            OpCodes::Equals => OpCodes::process_equals(program, param_modes),
            OpCodes::Halt => {
                debug!("{:?} => HALTING", thread::current().name());
                false
            },
        }
    }

    fn process_add(program: &mut Program, param_modes: &Vec<i32>) -> bool {
        let op1 = OpCodes::get_parameter(param_modes[0], program);
        let op2 = OpCodes::get_parameter(param_modes[1], program);
        let location = program.next() as usize;
        debug!("{:?} => ADD {} and {}", thread::current().name(), op1, op2);
        program.set_position(location, op1 + op2);
        true
    }

    fn process_multiply(program: &mut Program, param_modes: &Vec<i32>) -> bool {
        let op1 = OpCodes::get_parameter(param_modes[0], program);
        let op2 = OpCodes::get_parameter(param_modes[1], program);
        let location = program.next() as usize;
        debug!("{:?} => MULTIPLY {} and {}", thread::current().name(), op1, op2);
        program.set_position(location, op1 * op2);
        true
    }

    fn process_input(program: &mut Program, _param_modes: &Vec<i32>) -> bool {
        let location = program.next() as usize;
        program.set_position_from_input(location);
        true
    }

    fn process_output(program: &mut Program, param_modes: &Vec<i32>) -> bool {
        let data = OpCodes::get_parameter(param_modes[0], program);
        debug!("{:?} => OUTPUT : {}", thread::current().name(), data);
        program.to_output(data);
        true
    }

    fn process_jump_if_true(program: &mut Program, param_modes: &Vec<i32>) -> bool {
        let op1 = OpCodes::get_parameter(param_modes[0], program);
        let op2 = OpCodes::get_parameter(param_modes[1], program);
        if op1 != 0 {
            debug!("{:?} => Jump to {}", thread::current().name(), op2);
            program.set_pointer(op2 as usize);
        } else {
            debug!("{:?} => No Jump", thread::current().name());
        }
        true
    }

    fn process_jump_if_false(program: &mut Program, param_modes: &Vec<i32>) -> bool {
        let op1 = OpCodes::get_parameter(param_modes[0], program);
        let op2 = OpCodes::get_parameter(param_modes[1], program);
        if op1 == 0 {
            debug!("{:?} => Jump to {}", thread::current().name(), op2);
            program.set_pointer(op2 as usize);
        } else {
            debug!("{:?} => No Jump", thread::current().name());
        }
        true
    }

    fn process_less_than(program: &mut Program, param_modes: &Vec<i32>) -> bool {
        let op1 = OpCodes::get_parameter(param_modes[0], program);
        let op2 = OpCodes::get_parameter(param_modes[1], program);
        let location = program.next() as usize;
        debug!("{:?} =>  {} LT? {}", thread::current().name(), op1, op2);
        if op1 < op2 {
            program.set_position(location, 1);
        } else {
            program.set_position(location, 0);
        }
        true
    }

    fn process_equals(program: &mut Program, param_modes: &Vec<i32>) -> bool {
        let op1 = OpCodes::get_parameter(param_modes[0], program);
        let op2 = OpCodes::get_parameter(param_modes[1], program);
        let location = program.next() as usize;
        debug!("{:?} =>  {} EQ? {}", thread::current().name(), op1, op2);
        if op1 == op2 {
            program.set_position(location, 1);
        } else {
            program.set_position(location, 0);
        }
        true
    }

    fn get_parameter(parameter_mode: i32, program: &mut Program) -> i32 {
        let parameter = program.next();
        match parameter_mode {
            0 => program.at_position(parameter as usize),
            1 => parameter,
            _ => panic!("Invalid parameter mode!"),
        }
    }
}

enum Action {
    HALT,
    CONTINUE,
}

pub fn process(program: &mut Program) {
    loop {
        let instruction = Instruction::new(program.next());
        match instruction.process(program) {
            Action::HALT => break,
            _ => (),
        }
    }
}
