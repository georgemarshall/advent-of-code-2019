use crate::intcode::{parse_program, IntcodeMachine};

#[aoc_generator(day5)]
fn load_program(input: &str) -> Vec<i64> {
    parse_program(input).unwrap()
}

#[aoc(day5, part1)]
fn part1(program: &[i64]) -> String {
    let mut im = IntcodeMachine::new(program);
    im.input_push(1);
    im.run();
    format!("{:?}", im.output_buf())
}

#[aoc(day5, part2)]
fn part2(program: &[i64]) -> String {
    let mut im = IntcodeMachine::new(program);
    im.input_push(5);
    im.run();
    format!("{:?}", im.output_buf())
}