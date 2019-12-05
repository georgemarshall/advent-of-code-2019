use std::ops::Range;

struct NumberDigits {
    number: u32,
}

impl NumberDigits {
    fn new(number: u32) -> Self {
        NumberDigits { number }
    }
}

impl Iterator for NumberDigits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.number > 0 {
            let digit = (self.number % 10) as u8;
            self.number /= 10;
            Some(digit)
        } else {
            None
        }
    }
}

fn password_heuristic(password: u32) -> [u8; 10] {
    NumberDigits::new(password)
        // Check each digit is not greater than the last (reverse order)
        .scan(None, |last, d| {
            if let Some(l) = last {
                if d > *l {
                    return None;
                }
            }
            *last = Some(d);
            *last
        })
        // Count the occurrence of each digit
        .fold([0; 10], |mut acc, d| {
            acc[d as usize % 10] += 1;
            acc
        })
}

fn password_2_or_more(password: u32) -> bool {
    let heuristic = password_heuristic(password);
    heuristic.iter().sum::<u8>() == 6 && heuristic.iter().any(|&c| c >= 2)
}

fn password_has_double(password: u32) -> bool {
    let heuristic = password_heuristic(password);
    heuristic.iter().sum::<u8>() == 6 && heuristic.iter().any(|&c| c == 2)
}

#[aoc_generator(day4)]
fn password_range(input: &str) -> Range<u32> {
    let mut parts = input.split('-').filter_map(|s| s.parse().ok());
    Range {
        start: parts.next().unwrap(),
        end: parts.next().unwrap(),
    }
}

#[aoc(day4, part1)]
fn total_password_2_or_more(range: &Range<u32>) -> usize {
    range.to_owned().filter(|&p| password_2_or_more(p)).count()
}

#[aoc(day4, part2)]
fn total_password_has_double(range: &Range<u32>) -> usize {
    range.to_owned().filter(|&p| password_has_double(p)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_valid() {
        assert_eq!(password_2_or_more(111111), true);
        assert_eq!(password_2_or_more(223450), false);
        assert_eq!(password_2_or_more(123789), false);
    }

    #[test]
    fn test_part2_valid() {
        assert_eq!(password_has_double(112233), true);
        assert_eq!(password_has_double(123444), false);
        assert_eq!(password_has_double(111122), true);
    }
}
