use advent_of_code_2019::intcode_computer;
use advent_of_code_2019::intcode_computer::IO;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    env_logger::init();
    let mut input = fs::read_to_string("resources/day7.input").unwrap();
    input.pop();

    let mut instructions = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Max Thrust : {}", try_phase_settings(&mut instructions));
    println!(
        "Max Thrust with feedback : {}",
        try_settings(generate_phase_settings(), &mut instructions)
    );
}

fn try_settings(phase_settings: Vec<Vec<i32>>, instructions: &mut Vec<i32>) -> i32 {
    let mut max_thrust = -1;
    for phase_setting in phase_settings {
        let thrust = spawn_amplifiers(phase_setting, instructions);
        if max_thrust < thrust {
            max_thrust = thrust
        }
    }
    max_thrust
}

fn generate_phase_settings() -> Vec<Vec<i32>> {
    let mut visited = vec![-1; 5];
    let mut used = vec![false; 5];
    let mut settings = Vec::new();

    fn generate_phase_settings_rec(
        n: i32,
        visited: &mut Vec<i32>,
        used: &mut Vec<bool>,
        settings: &mut Vec<Vec<i32>>,
    ) {
        if n == 0 {
            settings.push(visited.clone());
            return;
        }
        for i in 0..5 {
            if !used[i] {
                used[i] = true;
                visited[(n - 1) as usize] = (i + 5) as i32;
                generate_phase_settings_rec(n - 1, visited, used, settings);
                used[i] = false;
            }
        }
    }

    generate_phase_settings_rec(5, &mut visited, &mut used, &mut settings);
    settings
}

fn spawn_amplifiers(phase_settings: Vec<i32>, instructions: &Vec<i32>) -> i32 {
    let (channels, initiator) = get_chained_channels(phase_settings);

    initiator.send(0).unwrap();
    let mut final_res = None;
    let mut i = 0;
    for channel in channels {
        let (input, output) = channel;
        let (overflow, overflow_rc) = mpsc::channel();

        struct InOutput {
            input: Receiver<i32>,
            output: Sender<i32>,
            overflow: Sender<i32>,
        }

        impl IO for InOutput {
            fn read(&mut self) -> i32 {
                self.input.recv().unwrap()
            }

            fn write(&mut self, data: i32) {
                let res = self.output.send(data);
                if res.is_err() {
                    self.overflow.send(data).unwrap();
                }
            }
        }

        let io = Box::new(InOutput {
            input,
            output,
            overflow,
        });

        let ins_clone = instructions.clone();

        thread::Builder::new()
            .name(i.to_string())
            .spawn(move || {
                amplifier_controller_executor(io, ins_clone);
                -1
            })
            .unwrap();
        i = i + 1;
        final_res = Some(overflow_rc);
    }

    final_res.unwrap().recv().unwrap()
}

fn get_chained_channels(
    phase_settings: Vec<i32>,
) -> (Vec<(Receiver<i32>, Sender<i32>)>, Sender<i32>) {
    let (old_sender, mut old_receiver) = mpsc::channel();
    let mut channels = Vec::new();

    old_sender.send(phase_settings[0]).unwrap();
    for i in 1..phase_settings.len() {
        let (sender, receiver) = mpsc::channel();
        sender.send(phase_settings[i]).unwrap();
        channels.push((old_receiver, sender));
        old_receiver = receiver;
    }
    let first_sender = old_sender.clone();
    channels.push((old_receiver, old_sender));

    (channels, first_sender)
}

fn try_phase_settings(instructions: &mut Vec<i32>) -> i32 {
    let mut available_phases = vec![true; 5];

    fn try_phase_settings_rec(
        input: i32,
        amp_id: i32,
        available_phases: &mut Vec<bool>,
        instructions: &mut Vec<i32>,
    ) -> i32 {
        if amp_id > 5 {
            return input;
        }
        let mut max_to_thrusters = -1;
        for i in 0..5 {
            if available_phases[i] {
                available_phases[i] = false;
                #[derive(Clone)]
                struct InOutput {
                    input: Vec<i32>,
                    output: Rc<RefCell<Vec<i32>>>,
                };

                impl IO for InOutput {
                    fn read(&mut self) -> i32 {
                        let x = self.input.pop();
                        x.unwrap()
                    }

                    fn write(&mut self, data: i32) {
                        self.output.borrow_mut().push(data);
                    }
                }

                let output = Rc::new(RefCell::new(Vec::new()));
                let out_c = output.clone();

                let io = Box::new(InOutput {
                    input: vec![input, i as i32],
                    output,
                });

                amplifier_controller_executor(io, instructions.clone());
                let output = out_c.borrow_mut().pop();
                let output = output.unwrap();
                let max =
                    try_phase_settings_rec(output, amp_id + 1, available_phases, instructions);
                if max > max_to_thrusters {
                    max_to_thrusters = max;
                }
                available_phases[i] = true;
            }
        }
        max_to_thrusters
    }

    try_phase_settings_rec(0, 1, &mut available_phases, instructions)
}

fn amplifier_controller_executor(io: Box<dyn IO>, instructions: Vec<i32>) {
    let mut program = intcode_computer::Program::new(instructions, Some(io));

    intcode_computer::process(&mut program);
}
