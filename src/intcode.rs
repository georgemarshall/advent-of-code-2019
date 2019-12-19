use std::error;
use std::fmt;
use std::fmt::Debug;
use std::sync::mpsc::{Receiver, RecvError, SendError, Sender};

// Total size 4096 * 8 = 32,768
const MEMORY: usize = 4096;

pub fn parse_program(s: &str) -> Option<Vec<i64>> {
    s.lines()
        .map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect())
        .next()
}

pub fn print_instruction(im: &IntcodeMachine, pc: usize, instruction: &Instruction) -> String {
    let offset = match instruction {
        Instruction::Add(_, _, _) => 4,
        Instruction::Multiply(_, _, _) => 4,
        Instruction::Input(_) => 2,
        Instruction::Output(_) => 2,
        Instruction::JumpIfTrue(_, _) => 3,
        Instruction::JumpIfFalse(_, _) => 3,
        Instruction::LessThan(_, _, _) => 4,
        Instruction::Equals(_, _, _) => 4,
        Instruction::RelativeBase(_) => 2,
        Instruction::Exit => 1,
    };

    let instruction = format!("{:?}", instruction);
    let bytes = format!("{:>5?}", &im.mem[pc..pc + offset]);

    format!("{:>5}: {:26} {}", pc, instruction, bytes)
}

type DebugHook = fn(&mut IntcodeMachine, usize, Instruction) -> Instruction;

#[derive(Debug)]
enum Error<T> {
    Recv(RecvError),
    Send(SendError<T>),
}

impl<T: fmt::Debug + Send> error::Error for Error<T> {
    fn description(&self) -> &str {
        match *self {
            Error::Recv(ref inner) => inner.description(),
            Error::Send(ref inner) => inner.description(),
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Recv(ref inner) => inner.source(),
            Error::Send(ref inner) => inner.source(),
        }
    }
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Recv(ref inner) => fmt::Display::fmt(inner, f),
            Error::Send(ref inner) => fmt::Display::fmt(inner, f),
        }
    }
}

impl<T> From<RecvError> for Error<T> {
    fn from(recv_error: RecvError) -> Self {
        Error::Recv(recv_error)
    }
}

impl<T> From<SendError<T>> for Error<T> {
    fn from(send_error: SendError<T>) -> Self {
        Error::Send(send_error)
    }
}

enum Mode {
    Position,
    Immediate,
    Relative,
}

enum Perm {
    Read,
    Write,
}

impl From<i64> for Mode {
    fn from(mode: i64) -> Self {
        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Add(i64, i64, i64),
    Multiply(i64, i64, i64),
    Input(i64),
    Output(i64),
    JumpIfTrue(i64, i64),
    JumpIfFalse(i64, i64),
    LessThan(i64, i64, i64),
    Equals(i64, i64, i64),
    RelativeBase(i64),
    Exit,
}

impl From<&mut IntcodeMachine> for Instruction {
    fn from(machine: &mut IntcodeMachine) -> Self {
        use Mode::{Immediate, Position, Relative};
        use Perm::{Read, Write};

        let instruction = machine.next();

        let opcode = instruction % 100;
        let mut mode = instruction / 100;

        let mut next = |perm| {
            let v = machine.next();
            let m = mode % 10;
            mode /= 10;

            match (m.into(), perm) {
                (Position, Read) => machine.load(v as usize),
                (Relative, Read) => machine.load((machine.relative_base + v) as usize),
                (Immediate, _) | (Position, Write) => v,
                (Relative, Write) => machine.relative_base + v,
            }
        };

        match opcode {
            1 => Instruction::Add(next(Read), next(Read), next(Write)),
            2 => Instruction::Multiply(next(Read), next(Read), next(Write)),
            3 => Instruction::Input(next(Write)),
            4 => Instruction::Output(next(Read)),
            5 => Instruction::JumpIfTrue(next(Read), next(Read)),
            6 => Instruction::JumpIfFalse(next(Read), next(Read)),
            7 => Instruction::LessThan(next(Read), next(Read), next(Write)),
            8 => Instruction::Equals(next(Read), next(Read), next(Write)),
            9 => Instruction::RelativeBase(next(Read)),
            99 => Instruction::Exit,
            _ => unreachable!(),
        }
    }
}

pub struct IntcodeMachine {
    pc: usize,
    pub mem: [i64; MEMORY],
    relative_base: i64,
    input: Option<Receiver<i64>>,
    output: Option<Sender<i64>>,
    debug: Option<DebugHook>,
    halted: bool,
}

impl IntcodeMachine {
    pub fn new(program: &[i64], input: Option<Receiver<i64>>, output: Option<Sender<i64>>) -> Self {
        // Initialize system memory
        let mut mem = [0; MEMORY];

        // Load the program into memory
        mem[..program.len()].copy_from_slice(program);

        IntcodeMachine {
            pc: 0,
            mem,
            relative_base: 0,
            input,
            output,
            debug: None,
            halted: false,
        }
    }

    pub fn load(&self, address: usize) -> i64 {
        self.mem[address]
    }

    pub fn store(&mut self, address: usize, v: i64) {
        self.mem[address] = v;
    }

    /// Run the intcode machine until it becomes halted.
    pub fn run(&mut self) {
        while !self.halted {
            let result = self.tick();

            if result.is_err() {
                break;
            }
        }

        // Drop input and output channels
        if let Some(rx_input) = self.input.take() {
            drop(rx_input);
        }
        if let Some(tx_output) = self.output.take() {
            drop(tx_output);
        }
    }

    pub fn set_debug(&mut self, hook: DebugHook) {
        self.debug = Some(hook)
    }

    pub fn unset_debug(&mut self) {
        self.debug = None
    }

    fn next(&mut self) -> i64 {
        let v = self.load(self.pc);
        self.pc += 1;
        v
    }

    fn tick(&mut self) -> Result<(), Error<i64>> {
        let pc = self.pc;
        let mut instruction: Instruction = self.into();

        if let Some(debug) = self.debug {
            instruction = debug(self, pc, instruction);
        }

        match instruction {
            Instruction::Add(r1, r2, r3) => {
                self.store(r3 as usize, r1 + r2);
            }
            Instruction::Multiply(r1, r2, r3) => {
                self.store(r3 as usize, r1 * r2);
            }
            Instruction::Input(r1) => {
                let rx_input = self.input.as_ref().expect("Input channel expected");
                let v = rx_input.recv()?;
                self.store(r1 as usize, v);
            }
            Instruction::Output(r1) => {
                let tx_output = self.output.as_ref().expect("Output channel expected");
                tx_output.send(r1)?;
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
            Instruction::RelativeBase(r1) => {
                self.relative_base += r1;
            }
            Instruction::Exit => {
                self.halted = true;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::sync::mpsc::channel;

    #[test]
    fn test_program_from_str() {
        let program = parse_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0\n");
        assert!(program.is_some());
        assert_eq!(
            program,
            Some(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ])
        );
    }

    // Day 2 examples
    #[test]
    fn test_intcode_machine() {
        let program = vec![1, 0, 0, 0, 99];
        let mut im = IntcodeMachine::new(&program, None, None);
        im.run();
        assert_eq!(&im.mem[..program.len()], &[2, 0, 0, 0, 99]);

        let program = vec![2, 3, 0, 3, 99];
        let mut im = IntcodeMachine::new(&program, None, None);
        im.run();
        assert_eq!(&im.mem[..program.len()], &[2, 3, 0, 6, 99]);

        let program = vec![2, 4, 4, 5, 99, 0];
        let mut im = IntcodeMachine::new(&program, None, None);
        im.run();
        assert_eq!(&im.mem[..program.len()], &[2, 4, 4, 5, 99, 9801]);

        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut im = IntcodeMachine::new(&program, None, None);
        im.run();
        assert_eq!(&im.mem[..program.len()], &[30, 1, 1, 4, 2, 5, 6, 0, 99]);

        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut im = IntcodeMachine::new(&program, None, None);
        im.run();
        assert_eq!(
            &im.mem[..program.len()],
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    // Day 5 examples
    #[test]
    fn test_input_output() {
        let program = vec![3, 0, 4, 0, 99];

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(1).unwrap();
        im.run();

        assert_eq!(rx_output.iter().collect_vec(), vec![1]);
    }

    #[test]
    fn test_immediate_mode() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut im = IntcodeMachine::new(&program, None, None);
        im.run();
        assert_eq!(&im.mem[..program.len()], &[1002, 4, 3, 4, 99]);

        let program = vec![1101, 100, -1, 4, 0];
        let mut im = IntcodeMachine::new(&program, None, None);
        im.run();
        assert_eq!(&im.mem[..program.len()], &[1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_conditional() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(8).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![1]);

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(1).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![0]);

        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(8).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![0]);

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(1).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![1]);

        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(8).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![1]);

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(1).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![0]);

        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(8).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![0]);

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(1).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![1]);
    }

    #[test]
    fn test_conditional_jump() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(1).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![1]);

        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(1).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![1]);

        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(1).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![999]);

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(8).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![1000]);

        let (tx_input, rx_input) = channel();
        let (tx_output, rx_output) = channel();
        let mut im = IntcodeMachine::new(&program, Some(rx_input), Some(tx_output));
        tx_input.send(50).unwrap();
        im.run();
        assert_eq!(rx_output.iter().collect_vec(), vec![1001]);
    }

    // Day 9 examples
    #[test]
    fn test_relative_mode() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let (tx_output, rx_output) = channel();

        let mut im = IntcodeMachine::new(&program, None, Some(tx_output));
        im.run();

        let output = rx_output.iter().collect_vec();
        assert_eq!(output, program);

        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let (tx_output, rx_output) = channel();

        let mut im = IntcodeMachine::new(&program, None, Some(tx_output));
        im.run();

        assert_eq!(rx_output.recv(), Ok(1219070632396864));

        let program = vec![104, 1125899906842624, 99];
        let (tx_output, rx_output) = channel();

        let mut im = IntcodeMachine::new(&program, None, Some(tx_output));
        im.run();

        assert_eq!(rx_output.recv(), Ok(1125899906842624));
    }
}
