struct Instruction<'a> {
    machine: &'a mut IntcodeMachine,
    opcode: usize,
    mode: usize,
}

impl<'a> Instruction<'a> {
    fn new(machine: &'a mut IntcodeMachine) -> Self {
        let instruction = machine.next();
        Instruction {
            machine,
            opcode: instruction % 100,
            mode: instruction / 100,
        }
    }

    fn execute(&mut self) {
        match self.opcode {
            1 => {
                let (r1, r2, r3) = (self.next(), self.next(), self.next_ptr());
                self.add(r1, r2, r3)
            }
            2 => {
                let (r1, r2, r3) = (self.next(), self.next(), self.next_ptr());
                self.multiply(r1, r2, r3)
            }
            3 => {
                let r1 = self.next_ptr();
                self.input(r1)
            }
            4 => {
                let r1 = self.next();
                self.output(r1)
            }
            5 => {
                let (r1, r2) = (self.next(), self.next());
                self.jump_if_true(r1, r2)
            }
            6 => {
                let (r1, r2) = (self.next(), self.next());
                self.jump_if_false(r1, r2)
            }
            7 => {
                let (r1, r2, r3) = (self.next(), self.next(), self.next_ptr());
                self.less_than(r1, r2, r3)
            }
            8 => {
                let (r1, r2, r3) = (self.next(), self.next(), self.next_ptr());
                self.equals(r1, r2, r3)
            }
            99 => self.exit(),
            _ => unreachable!(),
        }
    }

    fn next(&mut self) -> i32 {
        let mode = self.mode % 10;
        self.mode /= 10;

        let v = self.next_ptr();
        match mode {
            0 => self.machine.load(v),
            1 => v as i32,
            _ => unreachable!(),
        }
    }

    fn next_ptr(&mut self) -> usize {
        self.machine.next()
    }

    /// Opcode: 1
    fn add(&mut self, r1: i32, r2: i32, r3: usize) {
        self.machine.store(r3, r1 + r2);
    }

    /// Opcode: 2
    fn multiply(&mut self, r1: i32, r2: i32, r3: usize) {
        self.machine.store(r3, r1 * r2);
    }

    /// Opcode: 3
    fn input(&mut self, r1: usize) {
        let v = self.machine.input.pop().expect("Input expected");
        self.machine.store(r1, v);
    }

    /// Opcode: 4
    fn output(&mut self, r1: i32) {
        self.machine.output.push(r1);
    }

    /// Opcode: 5
    fn jump_if_true(&mut self, r1: i32, r2: i32) {
        if r1 != 0 {
            self.machine.pc = r2 as usize;
        }
    }

    /// Opcode: 6
    fn jump_if_false(&mut self, r1: i32, r2: i32) {
        if r1 == 0 {
            self.machine.pc = r2 as usize;
        }
    }

    /// Opcode: 7
    fn less_than(&mut self, r1: i32, r2: i32, r3: usize) {
        self.machine.store(r3, if r1 < r2 { 1 } else { 0 });
    }

    /// Opcode: 8
    fn equals(&mut self, r1: i32, r2: i32, r3: usize) {
        self.machine.store(r3, if r1 == r2 { 1 } else { 0 });
    }

    /// Opcode: 99
    fn exit(&mut self) {
        self.machine.halted = true;
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

    fn next(&mut self) -> usize {
        let v = self.load(self.pc);
        self.pc += 1;
        v as usize
    }

    pub fn load(&self, address: usize) -> i32 {
        self.mem[address]
    }

    pub fn store(&mut self, address: usize, v: i32) {
        self.mem[address] = v;
    }

    fn tick(&mut self) {
        Instruction::new(self).execute();
    }
}

#[aoc_generator(day5)]
fn program(input: &str) -> Vec<i32> {
    input.split(',').filter_map(|s| s.parse().ok()).collect()
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
