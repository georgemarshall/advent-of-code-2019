use crate::intcode::{parse_program, IntcodeMachine};

#[aoc_generator(day2)]
fn load_program(input: &str) -> Vec<i64> {
    parse_program(input).unwrap()
}

#[aoc(day2, part1)]
fn restored_program_state(program: &[i64]) -> i64 {
    let (noun, verb) = (12, 2);

    let mut im = IntcodeMachine::new(program);
    im.store(1, noun);
    im.store(2, verb);
    im.run();
    im.load(0)
}

#[aoc(day2, part2)]
fn fuzz_program_state(program: &[i64]) -> i64 {
    #[allow(clippy::inconsistent_digit_grouping)]
    let target = 1969_07_20;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut im = IntcodeMachine::new(program);
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
