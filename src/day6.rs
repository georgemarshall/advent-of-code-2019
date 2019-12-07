use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn count_orbits(
    memoizer: &mut HashMap<String, u32>,
    orbits: &HashMap<String, String>,
    planet: &str,
) -> u32 {
    if let Some(&m) = memoizer.get(planet) {
        return m;
    }
    let v = if let Some(p) = orbits.get(planet) {
        1 + count_orbits(memoizer, orbits, p)
    } else {
        0
    };
    memoizer.insert(planet.to_owned(), v);
    v
}

fn shortest_transfer(planets: &HashMap<&str, HashSet<&str>>, start: &str, end: &str) -> u32 {
    let mut depth = 0;
    let mut queue = vec![vec![start]];
    let mut visited = HashSet::new();

    'outer: while let Some(mut planet_set) = queue.pop() {
        let mut next_set = HashSet::new();
        while let Some(planet) = planet_set.pop() {
            visited.insert(planet);
            if planet == end {
                break 'outer;
            }
            next_set.extend(planets[planet].iter());
        }
        queue.push(next_set.difference(&visited).cloned().collect());
        depth += 1;
    }
    depth
}

#[aoc_generator(day6)]
fn load_orbits(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|s| {
            let (parent, child) = s.split(')').map(|s| s.to_owned()).collect_tuple().unwrap();
            // Swap the pairs
            (child, parent)
        })
        .collect()
}

#[aoc(day6, part1)]
fn total_orbits(orbits: &HashMap<String, String>) -> u32 {
    let mut memoizer: HashMap<String, u32> = HashMap::new();
    let planets: HashSet<&str> = HashSet::from_iter(
        orbits
            .keys()
            .map(|k| k.as_str())
            .chain(orbits.values().map(|v| v.as_str())),
    );
    planets
        .into_iter()
        .map(|planet| count_orbits(&mut memoizer, orbits, planet))
        .sum()
}

#[aoc(day6, part2)]
fn orbital_transfers(orbits: &HashMap<String, String>) -> u32 {
    let planets: HashMap<&str, HashSet<&str>> =
        orbits
            .iter()
            .fold(HashMap::new(), |mut acc, (parent, child)| {
                acc.entry(parent).or_insert_with(HashSet::new).insert(child);
                acc.entry(child).or_insert_with(HashSet::new).insert(parent);
                acc
            });
    shortest_transfer(&planets, &orbits["YOU"], &orbits["SAN"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_orbits() {
        let o = load_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n");
        assert_eq!(total_orbits(&o), 42);
    }

    #[test]
    fn test_orbital_transfers() {
        let o =
            load_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n");
        assert_eq!(orbital_transfers(&o), 4);
    }
}
