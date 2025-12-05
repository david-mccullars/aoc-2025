extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, ingredients) = parse(input);
    Some(
        ingredients
            .iter()
            .filter(|id| ranges.iter().any(|(s, e)| (s..=e).contains(id)))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ranges, _) = parse(input);
    Some(merge_ranges(ranges).iter().map(|(s, e)| e - s + 1).sum())
}

fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by_key(|r| r.0);
    ranges.into_iter().fold(vec![], |mut merged, (s, e)| {
        match merged.last_mut() {
            Some(last) if s <= last.1 + 1 => last.1 = last.1.max(e),
            _ => merged.push((s, e)),
        }
        merged
    })
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    parser!(
        section(lines(usize "-" usize))
        section(lines(usize))
    )
    .parse(input)
    .expect("Failed to parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
