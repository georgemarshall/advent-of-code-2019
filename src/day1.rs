fn fuel_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn cumulative_fuel_for_mass(mass: i32) -> i32 {
    let remaining_mass = fuel_for_mass(mass);

    if remaining_mass > 0 {
        remaining_mass + cumulative_fuel_for_mass(remaining_mass)
    } else {
        0
    }
}

#[aoc_generator(day1)]
fn load_modules(input: &str) -> Vec<i32> {
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

#[aoc(day1, part1)]
fn total_fuel(modules: &[i32]) -> i32 {
    modules.iter().map(|&mass| fuel_for_mass(mass)).sum()
}

#[aoc(day1, part2)]
fn total_cumulative_fuel(modules: &[i32]) -> i32 {
    modules
        .iter()
        .map(|&mass| cumulative_fuel_for_mass(mass))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100756), 33583);
    }

    #[test]
    fn test_cumulative_fuel_for_mass() {
        assert_eq!(cumulative_fuel_for_mass(14), 2);
        assert_eq!(cumulative_fuel_for_mass(1969), 966);
        assert_eq!(cumulative_fuel_for_mass(100756), 50346);
    }
}
