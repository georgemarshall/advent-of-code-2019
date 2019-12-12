use itertools::Itertools;
use num::integer::Integer;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};

#[derive(Clone, Copy)]
struct CmpResult {
    x: Ordering,
    y: Ordering,
    z: Ordering,
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
}

impl Moon {
    fn abs(self) -> Self {
        Moon {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn cmp(&self, other: &Self) -> CmpResult {
        CmpResult {
            x: self.x.cmp(&other.x),
            y: self.y.cmp(&other.y),
            z: self.z.cmp(&other.z),
        }
    }

    fn sum(self) -> i32 {
        self.x + self.y + self.z
    }
}

impl AddAssign<Velocity> for Moon {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.vx;
        self.y += rhs.vy;
        self.z += rhs.vz;
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd)]
struct Velocity {
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Velocity {
    fn abs(self) -> Self {
        Velocity {
            vx: self.vx.abs(),
            vy: self.vy.abs(),
            vz: self.vz.abs(),
        }
    }

    fn sum(self) -> i32 {
        self.vx + self.vy + self.vz
    }
}

impl AddAssign<CmpResult> for Velocity {
    fn add_assign(&mut self, rhs: CmpResult) {
        self.vx += rhs.x as i32;
        self.vy += rhs.y as i32;
        self.vz += rhs.z as i32;
    }
}

impl SubAssign<CmpResult> for Velocity {
    fn sub_assign(&mut self, rhs: CmpResult) {
        self.vx -= rhs.x as i32;
        self.vy -= rhs.y as i32;
        self.vz -= rhs.z as i32;
    }
}

fn simulate_moon_axis(moon_axis: &[i32]) -> (usize, usize) {
    let mut moons = moon_axis.to_owned();
    let mut velocities = vec![0; moons.len()];

    let mut seen = HashMap::new();
    let mut steps = 0;
    loop {
        let state = (moons.to_owned(), velocities.to_owned());
        if let Some(&step) = seen.get(&state) {
            return (step, steps - step);
        }
        seen.insert(state, steps);

        // Apply gravity
        for i in 0..moons.len() {
            for j in (i + 1)..moons.len() {
                let diff = moons[i].cmp(&moons[j]);
                velocities[i] -= diff as i32;
                velocities[j] += diff as i32;
            }
        }

        // Apply velocity
        moons
            .iter_mut()
            .zip_eq(velocities.iter())
            .for_each(|(moon, &velocity)| {
                *moon += velocity;
            });
        steps += 1;
    }
}

#[aoc_generator(day12)]
fn load_moons(input: &str) -> Vec<Moon> {
    let re = Regex::new(r"^<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>$").unwrap();
    input
        .lines()
        .map(|s| re.captures(s).unwrap())
        .map(|re| Moon {
            x: re["x"].parse().unwrap(),
            y: re["y"].parse().unwrap(),
            z: re["z"].parse().unwrap(),
        })
        .collect()
}

#[aoc(day12, part1)]
fn total_system_energy(moons: &[Moon]) -> i32 {
    const STEPS: usize = 1000;

    let mut moons = moons.to_owned();
    let mut velocities = vec![Velocity::default(); moons.len()];

    for _ in 0..STEPS {
        // Apply gravity
        for i in 0..moons.len() {
            for j in (i + 1)..moons.len() {
                let diffs = moons[i].cmp(&moons[j]);
                velocities[i] -= diffs;
                velocities[j] += diffs;
            }
        }

        // Apply velocity
        moons
            .iter_mut()
            .zip_eq(velocities.iter())
            .for_each(|(moon, &velocity)| {
                *moon += velocity;
            });
    }

    moons
        .iter()
        .zip_eq(velocities.iter())
        .map(|(&moon, &velocity)| moon.abs().sum() * velocity.abs().sum())
        .sum()
}

#[aoc(day12, part2)]
fn equal_state(moons: &[Moon]) -> usize {
    let (x_step, x_diff) = simulate_moon_axis(&moons.iter().map(|m| m.x).collect_vec());
    let (y_step, y_diff) = simulate_moon_axis(&moons.iter().map(|m| m.y).collect_vec());
    let (z_step, z_diff) = simulate_moon_axis(&moons.iter().map(|m| m.z).collect_vec());

    let cycle = x_diff.lcm(&y_diff).lcm(&z_diff);

    (x_step + cycle).max(y_step + cycle).max(z_step + cycle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moons() -> Vec<Moon> {
        vec![
            Moon { x: -1, y: 0, z: 2 },
            Moon {
                x: 2,
                y: -10,
                z: -7,
            },
            Moon { x: 4, y: -8, z: 8 },
            Moon { x: 3, y: 5, z: -1 },
        ]
    }

    #[test]
    fn test_load_moons() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>\n";

        assert_eq!(load_moons(input), moons());
    }

    #[test]
    fn test_part1() {
        let moons = moons();
    }
}
