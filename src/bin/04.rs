extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(4);

const DELTA: [isize; 3] = [-1, 0, 1];

pub fn part_one(input: &str) -> Option<usize> {
    Some(Rolls::parse(input).liftable().count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rolls = Rolls::parse(input);
    let mut liftable: VecDeque<Pos> = rolls.liftable().copied().collect();
    let mut seen: HashSet<Pos> = liftable.iter().copied().collect();

    while let Some(to_remove) = liftable.pop_front() {
        rolls.rolls.remove(&to_remove);
        for pos2 in rolls.liftable_around(&to_remove) {
            if seen.insert(pos2.clone()) {
                liftable.push_back(pos2);
            }
        }
    }
    Some(seen.len())
}

struct Rolls {
    rolls: HashSet<Pos>,
}

impl Rolls {
    fn parse(input: &str) -> Self {
        let mut grid = parser!(grid_of(".@"))
            .parse(input)
            .expect("Failed to parse");
        Self {
            rolls: grid.take_all('@'),
        }
    }

    fn is_roll_liftable(&self, pos: &Pos) -> bool {
        DELTA
            .iter()
            .flat_map(|dx| {
                DELTA.iter().filter(|dy| {
                    (*dx != 0 || **dy != 0) && self.rolls.contains(&(pos.0 + *dx, pos.1 + *dy))
                })
            })
            .count()
            < 4
    }

    fn liftable(&self) -> impl Iterator<Item = &Pos> {
        self.rolls.iter().filter(|pos| self.is_roll_liftable(pos))
    }

    fn liftable_around(&self, pos: &Pos) -> impl Iterator<Item = Pos> {
        DELTA.iter().flat_map(|dx| {
            DELTA.iter().filter_map(|dy| {
                let pos2 = (pos.0 + *dx, pos.1 + dy);
                ((*dx != 0 || *dy != 0)
                    && self.rolls.contains(&pos2)
                    && self.is_roll_liftable(&pos2))
                .then_some(pos2)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
