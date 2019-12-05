use std::collections::HashSet;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, PartialEq)]
enum Vector {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

impl FromStr for Vector {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);

        match direction {
            "U" => Ok(Vector::Up(distance.parse()?)),
            "D" => Ok(Vector::Down(distance.parse()?)),
            "L" => Ok(Vector::Left(distance.parse()?)),
            "R" => Ok(Vector::Right(distance.parse()?)),
            _ => unreachable!(),
        }
    }
}

struct Wire {
    vectors: Vec<Vector>,
}

impl Wire {
    fn new(vectors: Vec<Vector>) -> Self {
        Wire { vectors }
    }

    fn as_traced_points(&self) -> Vec<Point> {
        self.vectors
            .iter()
            .scan(Point::default(), |origin, vector| {
                Some(match *vector {
                    Vector::Up(v) => (0..v)
                        .map(|_| {
                            origin.y += 1;
                            *origin
                        })
                        .collect::<Vec<Point>>(),
                    Vector::Down(v) => (0..v)
                        .map(|_| {
                            origin.y -= 1;
                            *origin
                        })
                        .collect::<Vec<Point>>(),
                    Vector::Left(v) => (0..v)
                        .map(|_| {
                            origin.x -= 1;
                            *origin
                        })
                        .collect::<Vec<Point>>(),
                    Vector::Right(v) => (0..v)
                        .map(|_| {
                            origin.x += 1;
                            *origin
                        })
                        .collect::<Vec<Point>>(),
                })
            })
            .flatten()
            .collect()
    }
}

#[aoc_generator(day3)]
fn wires(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|s| Wire::new(s.split(',').filter_map(|v| v.parse().ok()).collect()))
        .collect()
}

#[aoc(day3, part1)]
fn manhattan_distance(wires: &[Wire]) -> i32 {
    let mut w = wires.iter();
    let wire1 = w.next().unwrap().as_traced_points();
    let wire2 = w.next().unwrap().as_traced_points();

    let w1: HashSet<Point> = HashSet::from_iter(wire1.iter().cloned());
    let w2: HashSet<Point> = HashSet::from_iter(wire2.iter().cloned());
    let intersections = w1.intersection(&w2);
    let origin = Point::default();

    intersections.map(|p| p.distance(origin)).min().unwrap()
}

#[aoc(day3, part2)]
fn shortest_path(wires: &[Wire]) -> usize {
    let mut w = wires.iter();
    let wire1 = w.next().unwrap().as_traced_points();
    let wire2 = w.next().unwrap().as_traced_points();

    let w1: HashSet<Point> = HashSet::from_iter(wire1.iter().cloned());
    let w2: HashSet<Point> = HashSet::from_iter(wire2.iter().cloned());
    let intersections = w1.intersection(&w2);
    let origin = Point::default();

    intersections
        .filter(|&&p| p != origin)
        .map(|&p| {
            wire1.iter().position(|&v| v == p).unwrap()
                + wire2.iter().position(|&v| v == p).unwrap()
                + 2
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let wires = vec![
            Wire::new(vec![
                Vector::Right(75),
                Vector::Down(30),
                Vector::Right(83),
                Vector::Up(83),
                Vector::Left(12),
                Vector::Down(49),
                Vector::Right(71),
                Vector::Up(7),
                Vector::Left(72),
            ]),
            Wire::new(vec![
                Vector::Up(62),
                Vector::Right(66),
                Vector::Up(55),
                Vector::Right(34),
                Vector::Down(71),
                Vector::Right(55),
                Vector::Down(58),
                Vector::Right(83),
            ]),
        ];
        assert_eq!(manhattan_distance(&wires), 159);

        let wires = vec![
            Wire::new(vec![
                Vector::Right(98),
                Vector::Up(47),
                Vector::Right(26),
                Vector::Down(63),
                Vector::Right(33),
                Vector::Up(87),
                Vector::Left(62),
                Vector::Down(20),
                Vector::Right(33),
                Vector::Up(53),
                Vector::Right(51),
            ]),
            Wire::new(vec![
                Vector::Up(98),
                Vector::Right(91),
                Vector::Down(20),
                Vector::Right(16),
                Vector::Down(67),
                Vector::Right(40),
                Vector::Up(7),
                Vector::Right(15),
                Vector::Up(6),
                Vector::Right(7),
            ]),
        ];
        assert_eq!(manhattan_distance(&wires), 135);
    }

    #[test]
    fn test_part2() {
        let wires = vec![
            Wire::new(vec![
                Vector::Right(75),
                Vector::Down(30),
                Vector::Right(83),
                Vector::Up(83),
                Vector::Left(12),
                Vector::Down(49),
                Vector::Right(71),
                Vector::Up(7),
                Vector::Left(72),
            ]),
            Wire::new(vec![
                Vector::Up(62),
                Vector::Right(66),
                Vector::Up(55),
                Vector::Right(34),
                Vector::Down(71),
                Vector::Right(55),
                Vector::Down(58),
                Vector::Right(83),
            ]),
        ];
        assert_eq!(shortest_path(&wires), 610);

        let wires = vec![
            Wire::new(vec![
                Vector::Right(98),
                Vector::Up(47),
                Vector::Right(26),
                Vector::Down(63),
                Vector::Right(33),
                Vector::Up(87),
                Vector::Left(62),
                Vector::Down(20),
                Vector::Right(33),
                Vector::Up(53),
                Vector::Right(51),
            ]),
            Wire::new(vec![
                Vector::Up(98),
                Vector::Right(91),
                Vector::Down(20),
                Vector::Right(16),
                Vector::Down(67),
                Vector::Right(40),
                Vector::Up(7),
                Vector::Right(15),
                Vector::Up(6),
                Vector::Right(7),
            ]),
        ];
        assert_eq!(shortest_path(&wires), 410);
    }
}
