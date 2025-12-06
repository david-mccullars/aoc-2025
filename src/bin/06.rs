extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let (rows, ops) = parse_rows(input);
    let problems = (0..ops.len()).map(|i| rows.iter().map(|r| r[i]).collect());
    Some(solve(ops, problems))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ops, problems) = parse_columns(input);
    Some(solve(ops, problems.into_iter()))
}

fn parse_rows(input: &str) -> (Vec<Vec<usize>>, Vec<char>) {
    parser!(
        lines(" "* a:repeat_sep(usize, " "+) " "* => a)
        line(" "* a:repeat_sep(c:char_of("+*") => if c == 0 { '+' } else { '*' }, " "+) " "* => a)
    )
    .parse(input)
    .unwrap()
}

fn parse_columns(input: &str) -> (Vec<char>, Vec<Vec<usize>>) {
    let grid = pad_lines(input);
    let width = grid.first().map(|r| r.len()).unwrap_or(0);

    let mut ops = vec![];
    let mut problems: Vec<Vec<usize>> = vec![vec![]];

    for col in (0..width).rev() {
        let column: String = grid.iter().map(|row| row[col]).collect();
        if column.trim().is_empty() {
            continue;
        }

        if let Some(n) = parse_number(&column) {
            problems.last_mut().unwrap().push(n);
        }

        if let Some(op) = find_operator(&column) {
            ops.push(op);
            problems.push(vec![]);
        }
    }

    (ops, problems)
}

fn pad_lines(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect()
}

fn parse_number(s: &str) -> Option<usize> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        Some(digits.parse().unwrap())
    }
}

fn find_operator(s: &str) -> Option<char> {
    s.chars().find(|&c| c == '+' || c == '*')
}

fn solve(ops: Vec<char>, problems: impl Iterator<Item = Vec<usize>>) -> usize {
    ops.into_iter()
        .zip(problems)
        .map(|(op, nums)| apply_op(op, &nums))
        .sum()
}

fn apply_op(op: char, nums: &[usize]) -> usize {
    if op == '+' {
        nums.iter().sum()
    } else {
        nums.iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
