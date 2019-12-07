enum Instruction {
    Add(i32, i32, i32),
    Multiply(i32, i32, i32),
    Exit,
}

impl From<&mut IntcodeMachine> for Instruction {
    fn from(machine: &mut IntcodeMachine) -> Self {
        let opcode = machine.next();
        let mut load_next = || {
            let v = machine.next();
            machine.load(v as usize)
        };

        match opcode {
            1 => Instruction::Add(load_next(), load_next(), machine.next()),
            2 => Instruction::Multiply(load_next(), load_next(), machine.next()),
            99 => Instruction::Exit,
            _ => unreachable!(),
        }
    }
}

struct IntcodeMachine {
    pc: usize,
    mem: Vec<i32>,
    halted: bool,
}

impl IntcodeMachine {
    pub fn new(mem: Vec<i32>) -> Self {
        IntcodeMachine {
            pc: 0,
            mem,
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
            Instruction::Exit => {
                self.halted = true;
            }
        }
    }
}

#[aoc_generator(day2)]
fn program(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect())
        .next()
        .unwrap()
}

#[aoc(day2, part1)]
fn restored_program(program: &[i32]) -> i32 {
    let (noun, verb) = (12, 2);

    let mut im = IntcodeMachine::new(program.to_owned());
    im.store(1, noun);
    im.store(2, verb);
    im.run();
    im.load(0)
}

#[aoc(day2, part2)]
fn fuzzed_program(program: &[i32]) -> i32 {
    #[allow(clippy::inconsistent_digit_grouping)]
    let target = 1969_07_20;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut im = IntcodeMachine::new(program.to_owned());
            im.store(1, noun);
            im.store(2, verb);
            im.run();

            if im.load(0) == target {
                return 100 * noun + verb;
            }
        }
    }
    0
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
    fn test_intcode_machine() {
        let mut im = IntcodeMachine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        im.run();
        assert_eq!(im.mem, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}
