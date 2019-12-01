use std::io::{self, BufRead};

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

fn main() {
    println!("Reading input from stdin...\n");
    let stdin = io::stdin();
    let modules: Vec<i32> = stdin
        .lock()
        .lines()
        .filter_map(|s| s.ok())
        .filter_map(|s| s.parse().ok())
        .collect();

    let total_fuel: i32 = modules.iter().map(|&mass| fuel_for_mass(mass)).sum();
    let total_cumulative_fuel: i32 = modules
        .iter()
        .map(|&mass| cumulative_fuel_for_mass(mass))
        .sum();

    println!("===== Results =====");
    println!("Fuel for mass: {}", total_fuel);
    println!("Cumulative fuel for mass: {}", total_cumulative_fuel);
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
