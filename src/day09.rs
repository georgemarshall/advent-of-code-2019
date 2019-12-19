use crate::intcode::{parse_program, IntcodeMachine};
use std::sync::mpsc::{channel, RecvError};

#[aoc_generator(day9)]
fn load_program(input: &str) -> Vec<i64> {
    parse_program(input).unwrap()
}

#[aoc(day9, part1)]
fn part1(program: &[i64]) -> Result<i64, RecvError> {
    let (tx_input, rx_input) = channel();
    let (tx_output, rx_output) = channel();

    tx_input.send(1).unwrap();

    let mut im = IntcodeMachine::new(program, Some(rx_input), Some(tx_output));
    im.run();

    rx_output.recv()
}

#[aoc(day9, part2)]
fn part2(program: &[i64]) -> Result<i64, RecvError> {
    let (tx_input, rx_input) = channel();
    let (tx_output, rx_output) = channel();

    tx_input.send(2).unwrap();

    let mut im = IntcodeMachine::new(program, Some(rx_input), Some(tx_output));
    im.run();

    rx_output.recv()
}
