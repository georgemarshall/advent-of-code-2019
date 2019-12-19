use crate::intcode::{parse_program, IntcodeMachine};
use itertools::Itertools;
use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};
use std::thread;
use std::thread::JoinHandle;

struct AmplificationCircuit {
    amplifiers: Vec<JoinHandle<()>>,
    inputs: Vec<Sender<i64>>,
    output: Receiver<i64>,
}

impl AmplificationCircuit {
    fn new(program: &[i64], phases: Vec<i64>) -> Option<Self> {
        let last_index = phases.len().checked_sub(1)?;

        // Setup initial input channel for the chain
        let (tx_input, rx_input) = channel();
        let mut rx_output = None;

        let (amplifiers, inputs): (Vec<_>, Vec<_>) = phases
            .into_iter()
            .enumerate()
            .scan((Some(tx_input), Some(rx_input)), |(tx, rx), (i, v)| {
                let builder = thread::Builder::new().name(format!("Amplifier {}", i));

                // Setup an output for each instance
                let (tx_link, rx_link) = channel();

                let input = Some(rx.replace(rx_link)?);
                let output = Some(tx_link.clone());
                let mut im = IntcodeMachine::new(program, input, output);
                let amplifier = builder
                    .spawn(move || {
                        im.run();
                    })
                    .unwrap();

                // Seed amplifier initialization input
                if let Some(sender) = tx {
                    sender.send(v).ok()?;
                }
                // Grab the channel recv for the last amplifier
                if i == last_index {
                    rx_output = rx.take();
                }

                // Grab the channel send for the amplifier
                tx.replace(tx_link).map(|sender| (amplifier, sender))
            })
            .unzip();

        rx_output.map(|output| AmplificationCircuit {
            amplifiers,
            inputs,
            output,
        })
    }

    fn join_amplifies(self) -> Vec<thread::Result<()>> {
        self.amplifiers
            .into_iter()
            .map(|amplifier| amplifier.join())
            .collect()
    }

    fn send_input(&self, t: i64) -> Result<(), SendError<i64>> {
        self.inputs
            .first()
            .ok_or_else(|| SendError(0))
            .and_then(|sender| sender.send(t))
    }

    fn recv_output(&self) -> Result<i64, RecvError> {
        self.output.recv()
    }
}

fn amplification_circuit(program: &[i64], phases: Vec<i64>) -> Option<i64> {
    let circuit = AmplificationCircuit::new(&program, phases)?;
    circuit.send_input(0).ok()?;

    let last_output = circuit.recv_output();
    circuit.join_amplifies();
    last_output.ok()
}

fn feedback_loop(program: &[i64], phases: Vec<i64>) -> Option<i64> {
    let circuit = AmplificationCircuit::new(&program, phases)?;
    let mut last_output = 0;

    // Send initial input
    circuit.send_input(last_output).ok()?;

    // Loop until we stop receiving output
    while let Ok(output) = circuit.recv_output() {
        last_output = output;

        // Send next input and break if the amplifies have shutdown
        if circuit.send_input(last_output).is_err() {
            break;
        }
    }

    circuit.join_amplifies();

    Some(last_output)
}

#[aoc_generator(day7)]
fn load_program(input: &str) -> Vec<i64> {
    parse_program(input).unwrap()
}

#[aoc(day7, part1)]
fn max_amplification_circuit(program: &[i64]) -> Option<i64> {
    (0..=4)
        .permutations(5)
        .filter_map(|phases| amplification_circuit(&program, phases))
        .max()
}

#[aoc(day7, part2)]
fn max_feedback_loop(program: &[i64]) -> Option<i64> {
    (5..=9)
        .permutations(5)
        .filter_map(|phases| feedback_loop(program, phases))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplification_circuit() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phases = vec![4, 3, 2, 1, 0];
        assert_eq!(amplification_circuit(&program, phases), Some(43210));

        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phases = vec![0, 1, 2, 3, 4];
        assert_eq!(amplification_circuit(&program, phases), Some(54321));

        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phases = vec![1, 0, 4, 3, 2];
        assert_eq!(amplification_circuit(&program, phases), Some(65210));
    }

    #[test]
    fn test_feedback_loop() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phases = vec![9, 8, 7, 6, 5];
        assert_eq!(feedback_loop(&program, phases), Some(139629729));

        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phases = vec![9, 7, 8, 5, 6];
        assert_eq!(feedback_loop(&program, phases), Some(18216));
    }
}
