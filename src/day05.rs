use crate::intcode::{parse_program, IntcodeMachine};
use itertools::Itertools;
use std::sync::mpsc::channel;

#[aoc_generator(day5)]
fn load_program(input: &str) -> Vec<i64> {
    parse_program(input).unwrap()
}

#[aoc(day5, part1)]
fn part1(program: &[i64]) -> String {
    let (tx_input, rx_input) = channel();
    let (tx_output, rx_output) = channel();
    tx_input.send(1).unwrap();

    let mut im = IntcodeMachine::new(program, Some(rx_input), Some(tx_output));
    im.run();
    format!("{:?}", rx_output.iter().collect_vec())
}

#[aoc(day5, part2)]
fn part2(program: &[i64]) -> String {
    let (tx_input, rx_input) = channel();
    let (tx_output, rx_output) = channel();
    tx_input.send(5).unwrap();

    let mut im = IntcodeMachine::new(program, Some(rx_input), Some(tx_output));
    im.run();
    format!("{:?}", rx_output.iter().collect_vec())
}
