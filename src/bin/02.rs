extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .into_iter()
            .flat_map(|(a, b)| (a..=b).filter(is_valid_only2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .into_iter()
            .flat_map(|(a, b)| (a..=b).filter(is_valid_any))
            .sum(),
    )
}

fn is_valid_only2(n: &usize) -> bool {
    _is_valid(n, 2)
}

fn is_valid_any(n: &usize) -> bool {
    _is_valid(n, usize::MAX)
}

fn _is_valid(n: &usize, max_repeats: usize) -> bool {
    let n: Vec<usize> = digits(*n, 10).collect();
    (2..=std::cmp::min(max_repeats, n.len())).any(|repeats| {
        let d = n.len() / repeats;
        n.len() % repeats == 0
            && (1..repeats).all(|i| n[((i - 1) * d)..(i * d)] == n[(i * d)..((i + 1) * d)])
    })
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    parser!(line(repeat_sep(usize "-" usize, ",")))
        .parse(&input.replace("\n", ""))
        .expect("Failed to parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
