use std::io::{self, BufRead};

struct IntcodeMachine {
    mem: Vec<u32>,
    halted: bool,
    pc: usize,
}

impl IntcodeMachine {
    pub fn new(mem: Vec<u32>) -> Self {
        IntcodeMachine {
            mem,
            halted: false,
            pc: 0,
        }
    }

    /// Run the intcode machine until it becomes halted.
    pub fn run(&mut self) {
        while !self.halted {
            self.tick();
        }
    }

    pub fn load(&self, address: usize) -> usize {
        self.mem[address] as usize
    }

    pub fn store(&mut self, address: usize, v: u32) {
        self.mem[address] = v;
    }

    fn add(&mut self, r1: usize, r2: usize, r3: usize) {
        let v = self.load(r1) + self.load(r2);
        self.store(r3, v as u32);
        self.pc += 4;
    }

    fn multiply(&mut self, r1: usize, r2: usize, r3: usize) {
        let v = self.load(r1) * self.load(r2);
        self.store(r3, v as u32);
        self.pc += 4;
    }

    fn exit(&mut self) {
        self.halted = true;
    }

    fn tick(&mut self) {
        let pc = self.pc;

        match self.load(pc) {
            1 => self.add(self.load(pc + 1), self.load(pc + 2), self.load(pc + 3)),
            2 => self.multiply(self.load(pc + 1), self.load(pc + 2), self.load(pc + 3)),
            99 => self.exit(),
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("Reading input from stdin...\n");
    // TODO: add support for passing this value in.
    let target_output = 19_690_720;

    let stdin = io::stdin();
    let program: Vec<u32> = stdin
        .lock()
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| s.split(',').filter_map(|v| v.parse().ok()).collect())
        .next()
        .unwrap_or_default();

    let restored_program = {
        let (noun, verb) = (12, 2);
        let mut im = IntcodeMachine::new(program.clone());
        im.store(1, noun);
        im.store(2, verb);
        im.run();
        im.load(0)
    };

    let fuzzed_input: (u32, u32) = {
        let mut r = (0, 0);
        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                let mut im = IntcodeMachine::new(program.clone());
                im.store(1, noun);
                im.store(2, verb);
                im.run();
                if im.load(0) == target_output {
                    r = (noun, verb);
                    break 'outer;
                }
            }
        }
        r
    };

    println!("===== Results =====");
    println!("Restored program memory: {}", restored_program);
    println!(
        "Fuzzed program memory: noun = {0}, verb = {1}",
        fuzzed_input.0, fuzzed_input.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_machine() {
        let mut im = IntcodeMachine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        im.run();
        assert_eq!(im.mem, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}
