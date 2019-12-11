use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn angle(self, other: Self) -> f64 {
        let delta_y: f64 = (self.y - other.y).into();
        let delta_x: f64 = (other.x - self.x).into();
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

fn asteroid_with_max_los(asteroids: &[Point]) -> Option<(Point, usize)> {
    asteroids
        .iter()
        .map(|&origin| {
            let count = asteroids
                .iter()
                .map(|&asteroid| (origin.angle(asteroid) * 100_000.0) as i32)
                .unique()
                .count();
            (origin, count)
        })
        .max_by(|&a, &b| a.1.cmp(&b.1))
}

#[aoc_generator(day10)]
fn load_map(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .map(|(y, s)| {
            s.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some(Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    }),
                    _ => None,
                })
                .collect_vec()
        })
        .flatten()
        .collect()
}

#[aoc(day10, part1)]
fn max_los(asteroids: &[Point]) -> Option<usize> {
    let (_, max) = asteroid_with_max_los(asteroids)?;
    Some(max)
}

#[aoc(day10, part2)]
fn two_hundredth_asteroid(asteroids: &[Point]) -> Option<i32> {
    let (origin, _) = asteroid_with_max_los(asteroids)?;

    let mut radial_map = asteroids.iter().fold(HashMap::new(), |mut acc, &asteroid| {
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
    use super::*;

    #[test]
    fn test_parse() {
        let map = ".#..#\n.....\n#####\n....#\n...##\n";

        assert_eq!(
            load_map(map),
            vec![
                Point { x: 1, y: 0 },
                Point { x: 4, y: 0 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 2 },
                Point { x: 4, y: 2 },
                Point { x: 4, y: 3 },
                Point { x: 3, y: 4 },
                Point { x: 4, y: 4 }
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
