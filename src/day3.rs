use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[allow(clippy::many_single_char_names)]
    fn overlap(a: Self, b: Self, c: Self, d: Self) -> Option<Self> {
        let (a_x, a_y) = (a.x as f32, a.y as f32);
        let (b_x, b_y) = (b.x as f32, b.y as f32);
        let (c_x, c_y) = (c.x as f32, c.y as f32);
        let (d_x, d_y) = (d.x as f32, d.y as f32);

        let a1 = b_y - a_y;
        let b1 = b_x - a_x;
        let a2 = d_y - c_y;
        let b2 = d_x - c_x;

        let determinant = a2 * b1 - a1 * b2;

        let s = (-a1 * (a_x - c_x) + b1 * (a_y - c_y)) / determinant;
        let t = (b2 * (a_y - c_y) - a2 * (a_x - c_x)) / determinant;

        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            let x = (a_x + (t * b1)) as i32;
            let y = (a_y + (t * a1)) as i32;
            Some(Point::new(x, y))
        } else {
            None
        }
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

    fn as_points(&self) -> Vec<Point> {
        self.vectors
            .iter()
            .scan(Point::default(), |origin, vector| {
                match *vector {
                    Vector::Up(v) => origin.y += v as i32,
                    Vector::Down(v) => origin.y -= v as i32,
                    Vector::Left(v) => origin.x -= v as i32,
                    Vector::Right(v) => origin.x += v as i32,
                };
                Some(*origin)
            })
            .collect()
    }

    fn as_points_with_length(&self) -> Vec<(Point, i32)> {
        self.vectors
            .iter()
            .scan((Point::default(), 0), |(origin, distance), vector| {
                match *vector {
                    Vector::Up(v) => {
                        origin.y += v as i32;
                        *distance += v as i32;
                    }
                    Vector::Down(v) => {
                        origin.y -= v as i32;
                        *distance += v as i32;
                    }
                    Vector::Left(v) => {
                        origin.x -= v as i32;
                        *distance += v as i32;
                    }
                    Vector::Right(v) => {
                        origin.x += v as i32;
                        *distance += v as i32;
                    }
                };
                Some((*origin, *distance))
            })
            .collect()
    }

    fn intersections(&self, other: &Wire) -> Vec<Point> {
        let points1 = self.as_points();
        let points2 = other.as_points();

        points1
            .iter()
            .zip(points1[1..].iter())
            .map(|(&a, &b)| {
                points2
                    .iter()
                    .zip(points2[1..].iter())
                    .filter_map(|(&c, &d)| Point::overlap(a, b, c, d))
                    .collect_vec()
            })
            .flatten()
            .collect()
    }

    fn intersection_lengths(&self, other: &Wire) -> Vec<i32> {
        let points1 = self.as_points_with_length();
        let points2 = other.as_points_with_length();

        points1
            .iter()
            .zip(points1[1..].iter())
            .map(|(&(a, ad), &(b, _))| {
                points2
                    .iter()
                    .zip(points2[1..].iter())
                    .filter_map(|(&(c, cd), &(d, _))| {
                        let intersection = Point::overlap(a, b, c, d)?;
                        Some(ad + cd + a.distance(intersection) + c.distance(intersection))
                    })
                    .collect_vec()
            })
            .flatten()
            .collect()
    }
}

#[aoc_generator(day3)]
fn load_wires(input: &str) -> (Wire, Wire) {
    input
        .lines()
        .map(|s| Wire::new(s.split(',').filter_map(|v| v.parse().ok()).collect()))
        .collect_tuple()
        .unwrap()
}

#[aoc(day3, part1)]
fn manhattan_distance((wire1, wire2): &(Wire, Wire)) -> Option<i32> {
    let origin = Point::default();
    wire1
        .intersections(&wire2)
        .into_iter()
        .map(|p| p.distance(origin))
        .min()
}

#[aoc(day3, part2)]
fn shortest_path((wire1, wire2): &(Wire, Wire)) -> Option<i32> {
    wire1.intersection_lengths(&wire2).into_iter().min()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wire1() -> Wire {
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
        ])
    }

    fn wire2() -> Wire {
        Wire::new(vec![
            Vector::Up(62),
            Vector::Right(66),
            Vector::Up(55),
            Vector::Right(34),
            Vector::Down(71),
            Vector::Right(55),
            Vector::Down(58),
            Vector::Right(83),
        ])
    }

    fn wire3() -> Wire {
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
        ])
    }

    fn wire4() -> Wire {
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
        ])
    }

    #[test]
    fn test_parse() {
        let (w1, w2) =
            load_wires("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n");
        assert_eq!(w1.vectors, wire1().vectors);
        assert_eq!(w2.vectors, wire2().vectors);
    }

    #[test]
    fn test_find_intersections() {
        let (wire1, wire2) = (wire1(), wire2());
        assert_eq!(
            wire1.intersections(&wire2),
            vec![
                Point { x: 158, y: -12 },
                Point { x: 146, y: 46 },
                Point { x: 155, y: 4 },
                Point { x: 155, y: 11 },
            ]
        );

        let (wire1, wire2) = (wire3(), wire4());
        assert_eq!(
            wire1.intersections(&wire2),
            vec![
                Point { x: 107, y: 47 },
                Point { x: 124, y: 11 },
                Point { x: 157, y: 18 },
                Point { x: 107, y: 71 },
                Point { x: 107, y: 51 },
            ]
        );
    }

    #[test]
    fn test_manhattan_distance() {
        let wires = (wire1(), wire2());
        assert_eq!(manhattan_distance(&wires), Some(159));

        let wires = (wire3(), wire4());
        assert_eq!(manhattan_distance(&wires), Some(135));
    }

    #[test]
    fn test_part2() {
        let wires = (wire1(), wire2());
        assert_eq!(shortest_path(&wires), Some(610));

        let wires = (wire3(), wire4());
        assert_eq!(shortest_path(&wires), Some(410));
    }
}
