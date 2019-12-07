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
fn wires(input: &str) -> (Wire, Wire) {
    let mut wire_iter = input
        .lines()
        .map(|s| Wire::new(s.split(',').filter_map(|v| v.parse().ok()).collect()));
    (wire_iter.next().unwrap(), wire_iter.next().unwrap())
}

#[aoc(day3, part1)]
fn manhattan_distance(wires: &(Wire, Wire)) -> i32 {
    let (wire1, wire2) = (wires.0.as_traced_points(), wires.1.as_traced_points());

    let w1: HashSet<Point> = HashSet::from_iter(wire1.into_iter());
    let w2: HashSet<Point> = HashSet::from_iter(wire2.into_iter());
    let intersections = w1.intersection(&w2);
    let origin = Point::default();

    intersections.map(|p| p.distance(origin)).min().unwrap()
}

#[aoc(day3, part2)]
fn shortest_path(wires: &(Wire, Wire)) -> usize {
    let (wire1, wire2) = (wires.0.as_traced_points(), wires.1.as_traced_points());

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
    fn test_parse() {
        let (w1, w2) =
            wires("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n");
        assert_eq!(
            w1.vectors,
            vec![
                Vector::Right(75),
                Vector::Down(30),
                Vector::Right(83),
                Vector::Up(83),
                Vector::Left(12),
                Vector::Down(49),
                Vector::Right(71),
                Vector::Up(7),
                Vector::Left(72),
            ]
        );
        assert_eq!(
            w2.vectors,
            vec![
                Vector::Up(62),
                Vector::Right(66),
                Vector::Up(55),
                Vector::Right(34),
                Vector::Down(71),
                Vector::Right(55),
                Vector::Down(58),
                Vector::Right(83),
            ]
        );
    }

    #[test]
    fn test_manhattan_distance() {
        let wires = (
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
        );
        assert_eq!(manhattan_distance(&wires), 159);

        let wires = (
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
        );
        assert_eq!(manhattan_distance(&wires), 135);
    }

    #[test]
    fn test_part2() {
        let wires = (
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
        );
        assert_eq!(shortest_path(&wires), 610);

        let wires = (
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
        );
        assert_eq!(shortest_path(&wires), 410);
    }
}
