extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Manifold::parse(input).beam())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Manifold::parse(input).timelines())
}

struct Manifold {
    start: Pos,
    splitters: HashSet<Pos>,
    bounds: Pos,
    timelines_cache: HashMap<Pos, usize>,
}

impl Manifold {
    fn parse(input: &str) -> Self {
        let mut grid = parser!(grid_of(".S^"))
            .parse(input)
            .expect("Failed to parse");
        Self {
            start: grid.take_one('S'),
            splitters: grid.take_all('^'),
            bounds: grid.bounds,
            timelines_cache: HashMap::new(),
        }
    }

    fn beam(&self) -> usize {
        let mut splits = 0;
        let mut xs: HashSet<isize> = [self.start.0].into();
        for y in self.start.1..=self.bounds.1 {
            xs = xs
                .iter()
                .flat_map(|&x| {
                    if self.splitters.contains(&(x, y)) {
                        splits += 1;
                        vec![x - 1, x + 1]
                    } else {
                        vec![x]
                    }
                })
                .collect();
        }
        splits
    }

    fn timelines(&mut self) -> usize {
        self._timelines(self.start.clone())
    }

    fn _timelines(&mut self, pos: Pos) -> usize {
        if let Some(&t) = self.timelines_cache.get(&pos) {
            return t;
        }

        let t = if pos.1 > self.bounds.1 {
            1
        } else if self.splitters.contains(&pos) {
            self._timelines((pos.0 - 1, pos.1 + 1)) + self._timelines((pos.0 + 1, pos.1 + 1))
        } else {
            self._timelines((pos.0, pos.1 + 1))
        };

        self.timelines_cache.insert(pos.clone(), t);
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
