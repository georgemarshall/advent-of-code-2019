use crate::intcode::{parse_program, IntcodeMachine};
use ansi_term::Color as TermColor;
use itertools::Itertools;
use std::collections::HashMap;

const PIXEL: &str = "█";

#[derive(Copy, Clone)]
enum Color {
    Black,
    White,
}

impl From<i64> for Color {
    fn from(color: i64) -> Self {
        match color {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        }
    }
}

impl From<Color> for i64 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn translate_rotation(&mut self, rotation: Rotation) {
        *self = match rotation {
            Rotation::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Rotation::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
        };
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn translate_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
    }
}

enum Rotation {
    Left,
    Right,
}

impl From<i64> for Rotation {
    fn from(rotation: i64) -> Self {
        match rotation {
            0 => Rotation::Left,
            1 => Rotation::Right,
            _ => unreachable!(),
        }
    }
}

fn hull_painting_robot(program: &[i64], input: Color) -> HashMap<Point, Color> {
    let mut im = IntcodeMachine::new(program);
    im.input_push(input.into());

    let mut painted = HashMap::new();
    let mut direction = Direction::Up;
    let mut origin = Point::default();

    while let (Some(color), Some(rotation)) = (im.run_output(), im.run_output()) {
        // Set the painted color for the current position
        *painted.entry(origin.to_owned()).or_insert(Color::Black) = color.into();

        // Move to the next position
        direction.translate_rotation(rotation.into());
        origin.translate_direction(direction);

        // Find the input color of the next position
        let panel = *painted.entry(origin.to_owned()).or_insert(Color::Black);
        im.input_push(panel.into());
    }

    painted
}

fn render_painted(painted: HashMap<Point, Color>) -> Option<String> {
    let (x1_iter, x2_iter) = painted.keys().map(|p| p.x).tee();
    let (y1_iter, y2_iter) = painted.keys().map(|p| p.y).tee();

    // top right
    let x1 = x1_iter.max()?;
    let y1 = y1_iter.max()?;

    // bottom left
    let x2 = x2_iter.min()?;
    let y2 = y2_iter.min()?;

    let x_offset = x2.abs();
    let y_offset = y2.abs();

    let width = (x1.abs() + x_offset + 1) as usize;
    let height = (y1.abs() + y_offset + 1) as usize;

    let mut grid = vec![vec![Color::Black; width]; height];
    for (point, color) in painted {
        let x = (point.x + x_offset) as usize;
        let y = (height - 1) - (point.y + y_offset) as usize;

        grid[y][x] = color;
    }

    let lines = grid.into_iter().map(|row| {
        let mut line = String::from("\t");
        line.extend(row.into_iter().map(|color| {
            match color {
                Color::Black => TermColor::Black,
                Color::White => TermColor::White,
            }
            .paint(PIXEL)
            .to_string()
        }));
        line.push('\n');
        line
    });

    let mut output = String::from("\n\n");
    output.extend(lines);
    output.push('\n');
    Some(output)
}

#[aoc_generator(day11)]
fn load_program(input: &str) -> Vec<i64> {
    parse_program(input).unwrap()
}

#[aoc(day11, part1)]
fn unique_square(program: &[i64]) -> usize {
    let painted = hull_painting_robot(program, Color::Black);
    let panels = painted.len();
    print!("{}", render_painted(painted).unwrap());
    panels
}

#[aoc(day11, part2)]
fn unique_square2(program: &[i64]) -> Option<String> {
    let painted = hull_painting_robot(program, Color::White);

    render_painted(painted)
}
