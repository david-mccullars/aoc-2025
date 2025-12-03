extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    Some(total_joltage(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(total_joltage(input, 12))
}

fn total_joltage(input: &str, turn_on: usize) -> usize {
    parser!(lines(digit+))
        .parse(input)
        .expect("Failed to parse")
        .into_iter()
        .map(|bank| joltage(&bank, turn_on))
        .sum()
}

fn joltage(mut bank: &[usize], turn_on: usize) -> usize {
    (0..turn_on).fold(0, |num, i| {
        let d = bank[0..(bank.len() + i + 1 - turn_on)]
            .iter()
            .max()
            .unwrap();
        let i = bank.iter().position(|x| x == d).unwrap();
        bank = &bank[(i + 1)..];
        (num * 10) + d
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
