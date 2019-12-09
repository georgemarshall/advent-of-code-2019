use crate::intcode::{parse_program, IntcodeMachine};
use itertools::Itertools;
use rayon::prelude::*;

fn amplification_circuit(program: &[i64], phases: Vec<i64>) -> Option<i64> {
    phases
        .into_iter()
        .map(|phase| {
            let mut amplifier = IntcodeMachine::new(program);
            amplifier.input_push(phase);
            amplifier
        })
        .scan(0, |input, mut amplifier| {
            amplifier.input_push(*input);
            amplifier.run();

            *input = amplifier.output_pop()?;
            Some(*input)
        })
        .last()
}

fn feedback_loop(program: &[i64], phases: Vec<i64>) -> i64 {
    let mut amplifiers = phases
        .into_iter()
        .map(|phase| {
            let mut amplifier = IntcodeMachine::new(program);
            amplifier.input_push(phase);
            amplifier
        })
        .collect_vec();

    let mut last_output = 0;
    'feedback: loop {
        for amplifier in &mut amplifiers {
            amplifier.input_push(last_output);
            if let Some(output) = amplifier.run_output() {
                last_output = output;
            } else {
                break 'feedback;
            }
        }
    }
    last_output
}

#[aoc_generator(day7)]
fn load_program(input: &str) -> Vec<i64> {
    parse_program(input).unwrap()
}

#[aoc(day7, part1)]
fn max_amplification_circuit(program: &[i64]) -> Option<i64> {
    (0..=4)
        .permutations(5)
        .collect_vec()
        .into_par_iter()
        .filter_map(|phases| amplification_circuit(&program, phases))
        .max()
}

#[aoc(day7, part2)]
fn max_feedback_loop(program: &[i64]) -> Option<i64> {
    (5..=9)
        .permutations(5)
        .collect_vec()
        .into_par_iter()
        .map(|phases| feedback_loop(program, phases))
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
        assert_eq!(amplification_circuit(&program, phases).unwrap(), 43210);

        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phases = vec![0, 1, 2, 3, 4];
        assert_eq!(amplification_circuit(&program, phases).unwrap(), 54321);

        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phases = vec![1, 0, 4, 3, 2];
        assert_eq!(amplification_circuit(&program, phases).unwrap(), 65210);
    }

    #[test]
    fn test_feedback_loop() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phases: Vec<i64> = vec![9, 8, 7, 6, 5];
        assert_eq!(feedback_loop(&program, phases), 139629729);

        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phases: Vec<i64> = vec![9, 7, 8, 5, 6];
        assert_eq!(feedback_loop(&program, phases), 18216);
    }
}
