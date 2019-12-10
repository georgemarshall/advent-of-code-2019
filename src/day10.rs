use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn angle(self, other: Self) -> f32 {
        let delta_y = self.y as f32 - other.y as f32;
        let delta_x = other.x as f32 - self.x as f32;
        let result = delta_x.atan2(delta_y).to_degrees();
        if result < 0.0 {
            result + 360.0
        } else {
            result
        }
    }

    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn asteroids_from_map(map: &[Vec<Position>]) -> Vec<Point> {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, pos)| match *pos {
                    Position::Asteroid => Some(Point {
                        x: x as i32,
                        y: y as i32,
                    }),
                    _ => None,
                })
                .collect_vec()
        })
        .flatten()
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Position {
    Empty,
    Asteroid,
}

impl From<char> for Position {
    fn from(c: char) -> Self {
        match c {
            '.' => Position::Empty,
            '#' => Position::Asteroid,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day10)]
fn load_map(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .map(|s| s.chars().map(Position::from).collect())
        .collect()
}

#[aoc(day10, part1)]
fn max_los(map: &[Vec<Position>]) -> Option<usize> {
    let asteroids = asteroids_from_map(map);
    asteroids
        .iter()
        .map(|origin| {
            asteroids
                .iter()
                .skip_while(|&a| a == origin)
                .map(|&a| (origin.angle(a) * 100_000.0) as i32)
                .unique()
                .count()
        })
        .max()
}

#[aoc(day10, part2)]
fn two_hundredth_asteroid(map: &[Vec<Position>]) -> Option<i32> {
    let origin = Point { x: 23, y: 20 };

    let mut radial_map = asteroids_from_map(map)
        .into_iter()
        .skip_while(|&a| a == origin)
        .fold(HashMap::new(), |mut acc, asteroid| {
            let ang = (origin.angle(asteroid) * 100_000.0) as i32;
            acc.entry(ang).or_insert_with(Vec::new).push(asteroid);
            acc
        });

    // Sort all asteroids in descending order by distance from origin
    radial_map.values_mut().for_each(|v| {
        v.sort_by(|a, b| b.distance(origin).cmp(&a.distance(origin)));
    });

    // Copy all the radians, so we can run a cycling iteration on it
    let mut radians = radial_map.keys().copied().collect_vec();
    radians.sort();

    radians
        .iter()
        .cycle()
        .filter_map(|ang| radial_map.get_mut(ang)?.pop())
        .take(200)
        .last()
        .map(|asteroid| asteroid.x * 100 + asteroid.y)
}

#[cfg(test)]
mod tests {
    use super::Position::{Asteroid as A, Empty as E};
    use super::*;

    #[test]
    fn test_parse() {
        let map = ".#..#\n.....\n#####\n....#\n...##\n";

        assert_eq!(
            load_map(map),
            vec![
                vec![E, A, E, E, A],
                vec![E, E, E, E, E],
                vec![A, A, A, A, A],
                vec![E, E, E, E, A],
                vec![E, E, E, A, A]
            ]
        );
    }

    #[test]
    fn test_find_angle() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };

        assert_eq!(a.angle(b), 135.0);
        assert_eq!(b.angle(a), 315.0);
    }

    #[test]
    fn test_max_los() {
        let map = load_map(
            "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n",
        );
        assert_eq!(max_los(&map), Some(33));

        let map = load_map(
            "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.\n",
        );
        assert_eq!(max_los(&map), Some(35));

        let map = load_map(
            ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..\n",
        );
        assert_eq!(max_los(&map), Some(41));

        let map = load_map(
            ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##\n",
        );
        assert_eq!(max_los(&map), Some(210));
    }
}
