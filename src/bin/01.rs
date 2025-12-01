extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<isize> {
    let mut dial = Dial::new();
    for (dir, qty) in parse(input) {
        dial.rotate1(dir, qty);
    }
    Some(dial.clicks)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut dial = Dial::new();
    for (dir, qty) in parse(input) {
        dial.rotate2(dir, qty);
    }
    Some(dial.clicks)
}

fn parse(input: &str) -> Vec<(usize, isize)> {
    parser!(lines(char_of("LR") isize))
        .parse(input)
        .expect("Failed to parse")
}

struct Dial {
    value: isize,
    clicks: isize,
}

impl Dial {
    fn new() -> Self {
        Self {
            value: 50,
            clicks: 0,
        }
    }

    fn rotate1(&mut self, dir: usize, qty: isize) {
        if dir == 0 {
            self.value -= qty;
        } else {
            self.value += qty;
        }

        while self.value < 0 {
            self.value += 100;
        }
        self.value %= 100;

        if self.value == 0 {
            self.clicks += 1;
        }
    }

    fn rotate2(&mut self, dir: usize, qty: isize) {
        let was_on_zero = self.value == 0;
        self.clicks += qty / 100;
        if dir == 0 {
            self.value -= qty % 100;
        } else {
            self.value += qty % 100;
        }

        if qty > 0 && !was_on_zero && !(1..100).contains(&self.value) {
            self.clicks += 1;
        }

        while self.value < 0 {
            self.value += 100;
        }
        self.value %= 100;
    }
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
        assert_eq!(result, Some(6));
    }
}
