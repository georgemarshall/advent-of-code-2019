use itertools::Itertools;
use std::collections::LinkedList;

enum Mode {
    Position,
    Immediate,
}

impl From<i32> for Mode {
    fn from(mode: i32) -> Self {
        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Add(i32, i32, i32),
    Multiply(i32, i32, i32),
    Input(i32),
    Output(i32),
    JumpIfTrue(i32, i32),
    JumpIfFalse(i32, i32),
    LessThan(i32, i32, i32),
    Equals(i32, i32, i32),
    Exit,
}

impl From<&mut IntcodeMachine> for Instruction {
    fn from(machine: &mut IntcodeMachine) -> Self {
        let instruction = machine.next();

        let opcode = instruction % 100;

        let mut mode = instruction / 100;
        let mut mode_next = || {
            let v = machine.next();
            let m = mode % 10;
            mode /= 10;

            match m.into() {
                Mode::Position => machine.load(v as usize),
                Mode::Immediate => v,
            }
        };

        match opcode {
            1 => Instruction::Add(mode_next(), mode_next(), machine.next()),
            2 => Instruction::Multiply(mode_next(), mode_next(), machine.next()),
            3 => Instruction::Input(machine.next()),
            4 => Instruction::Output(mode_next()),
            5 => Instruction::JumpIfTrue(mode_next(), mode_next()),
            6 => Instruction::JumpIfFalse(mode_next(), mode_next()),
            7 => Instruction::LessThan(mode_next(), mode_next(), machine.next()),
            8 => Instruction::Equals(mode_next(), mode_next(), machine.next()),
            99 => Instruction::Exit,
            _ => unreachable!(),
        }
    }
}

struct IntcodeMachine {
    pc: usize,
    mem: Vec<i32>,
    input: LinkedList<i32>,
    output: LinkedList<i32>,
    halted: bool,
}

impl IntcodeMachine {
    pub fn new(mem: Vec<i32>) -> Self {
        IntcodeMachine {
            pc: 0,
            mem,
            input: LinkedList::new(),
            output: LinkedList::new(),
            halted: false,
        }
    }

    /// Run the intcode machine until it becomes halted.
    pub fn run(&mut self) {
        while !self.halted {
            self.tick();
        }
    }

    pub fn run_output(&mut self) -> Option<i32> {
        while !self.halted && self.output.is_empty() {
            self.tick();
        }
        self.output.pop_back()
    }

    fn push_input(&mut self, v: i32) {
        self.input.push_front(v)
    }

    fn next(&mut self) -> i32 {
        let v = self.load(self.pc);
        self.pc += 1;
        v
    }

    pub fn load(&self, address: usize) -> i32 {
        self.mem[address]
    }

    pub fn store(&mut self, address: usize, v: i32) {
        self.mem[address] = v;
    }

    fn tick(&mut self) {
        match self.into() {
            Instruction::Add(r1, r2, r3) => {
                self.store(r3 as usize, r1 + r2);
            }
            Instruction::Multiply(r1, r2, r3) => {
                self.store(r3 as usize, r1 * r2);
            }
            Instruction::Input(r1) => {
                let v = self.input.pop_back().expect("Input expected");
                self.store(r1 as usize, v);
            }
            Instruction::Output(r1) => {
                self.output.push_front(r1);
            }
            Instruction::JumpIfTrue(r1, r2) => {
                if r1 != 0 {
                    self.pc = r2 as usize;
                }
            }
            Instruction::JumpIfFalse(r1, r2) => {
                if r1 == 0 {
                    self.pc = r2 as usize;
                }
            }
            Instruction::LessThan(r1, r2, r3) => {
                self.store(r3 as usize, if r1 < r2 { 1 } else { 0 });
            }
            Instruction::Equals(r1, r2, r3) => {
                self.store(r3 as usize, if r1 == r2 { 1 } else { 0 });
            }
            Instruction::Exit => {
                self.halted = true;
            }
        }
    }
}

#[aoc_generator(day7)]
fn program(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect())
        .next()
        .unwrap()
}

#[aoc(day7, part1)]
fn amplification_circuit(program: &[i32]) -> Option<i32> {
    (0..=4)
        .permutations(5)
        .filter_map(|phases| {
            phases
                .into_iter()
                .map(|phase| {
                    let mut amplifier = IntcodeMachine::new(program.to_owned());
                    amplifier.push_input(phase);
                    amplifier
                })
                .scan(0, |input, mut amplifier| {
                    amplifier.push_input(*input);
                    amplifier.run();

                    *input = amplifier.output.pop_back()?;
                    Some(*input)
                })
                .last()
        })
        .max()
}

#[aoc(day7, part2)]
fn feedback_loop(program: &[i32]) -> Option<i32> {
    (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut amplifiers: Vec<IntcodeMachine> = phases
                .into_iter()
                .map(|phase| {
                    let mut amplifier = IntcodeMachine::new(program.to_owned());
                    amplifier.push_input(phase);
                    amplifier
                })
                .collect();

            let mut last_output = 0;
            'feedback: loop {
                for amplifier in &mut amplifiers {
                    amplifier.push_input(last_output);
                    if let Some(output) = amplifier.run_output() {
                        last_output = output;
                    } else {
                        break 'feedback;
                    }
                }
            }
            last_output
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let p = program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0\n");
        assert_eq!(
            p,
            vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]
        );
    }

    #[test]
    fn test_p2() {
        let p = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phases: Vec<i32> = vec![9, 8, 7, 6, 5];
        let mut amplifiers: Vec<IntcodeMachine> = phases
            .into_iter()
            .map(|phase| {
                let mut amplifier = IntcodeMachine::new(p.to_owned());
                amplifier.push_input(phase);
                amplifier
            })
            .collect();

        let mut last_output = 0;
        'outer: loop {
            for amplifier in &mut amplifiers {
                amplifier.push_input(last_output);
                if let Some(output) = amplifier.run_output() {
                    last_output = output;
                } else {
                    break 'outer;
                }
            }
        }

        assert_eq!(last_output, 139629729);
    }
}
