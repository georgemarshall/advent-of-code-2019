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
    input: Vec<i32>,
    output: Vec<i32>,
    halted: bool,
}

impl IntcodeMachine {
    pub fn new(mem: Vec<i32>, input: Vec<i32>) -> Self {
        IntcodeMachine {
            pc: 0,
            mem,
            input,
            output: Vec::new(),
            halted: false,
        }
    }

    /// Run the intcode machine until it becomes halted.
    pub fn run(&mut self) {
        while !self.halted {
            self.tick();
        }
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
                let v = self.input.pop().expect("Input expected");
                self.store(r1 as usize, v);
            }
            Instruction::Output(r1) => {
                self.output.push(r1);
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

#[aoc_generator(day5)]
fn program(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect())
        .next()
        .unwrap()
}

#[aoc(day5, part1)]
fn part1(program: &[i32]) -> String {
    let mut im = IntcodeMachine::new(program.to_owned(), vec![1]);
    im.run();
    format!("{:?}", im.output)
}

#[aoc(day5, part2)]
fn part2(program: &[i32]) -> String {
    let mut im = IntcodeMachine::new(program.to_owned(), vec![5]);
    im.run();
    format!("{:?}", im.output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let p = program("1,9,10,3,2,3,11,0,99,30,40,50\n");
        assert_eq!(p, vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_basic() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut im = IntcodeMachine::new(program, vec![]);
        im.run();
        assert_eq!(im.mem, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_input_output() {
        let program = vec![3, 0, 4, 0, 99];
        let mut im = IntcodeMachine::new(program, vec![1]);
        im.run();
        assert_eq!(im.output, vec![1]);
    }

    #[test]
    fn test_immediate_mode() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut im = IntcodeMachine::new(program, vec![]);
        im.run();
        assert_eq!(im.mem, vec![1002, 4, 3, 4, 99]);

        let program = vec![1101, 100, -1, 4, 0];
        let mut im = IntcodeMachine::new(program, vec![]);
        im.run();
        assert_eq!(im.mem, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_conditional_jump() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut im = IntcodeMachine::new(program, vec![1]);
        im.run();
        assert_eq!(im.output, vec![1]);

        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut im = IntcodeMachine::new(program, vec![1]);
        im.run();
        assert_eq!(im.output, vec![1]);

        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let mut im = IntcodeMachine::new(program.to_owned(), vec![1]);
        im.run();
        assert_eq!(im.output, vec![999]);

        let mut im = IntcodeMachine::new(program.to_owned(), vec![8]);
        im.run();
        assert_eq!(im.output, vec![1000]);

        let mut im = IntcodeMachine::new(program.to_owned(), vec![50]);
        im.run();
        assert_eq!(im.output, vec![1001]);
    }
}
