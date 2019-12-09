use crate::intcode::{parse_program, IntcodeMachine};

#[aoc_generator(day9)]
fn load_program(input: &str) -> Vec<i64> {
    parse_program(input).unwrap()
}

#[aoc(day9, part1)]
fn part1(program: &[i64]) -> Option<i64> {
    let mut im = IntcodeMachine::new(program);
    im.input_push(1);
    im.run();
    im.output_pop()
}

#[aoc(day9, part2)]
fn part2(program: &[i64]) -> Option<i64> {
    let mut im = IntcodeMachine::new(program);
    im.input_push(2);
    im.run();
    im.output_pop()
}
