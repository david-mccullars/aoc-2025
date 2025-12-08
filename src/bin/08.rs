extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

advent_of_code::solution!(8);

#[cfg(test)]
const BOXES: usize = 10;
#[cfg(not(test))]
const BOXES: usize = 1000;

pub fn part_one(input: &str) -> Option<usize> {
    let mut boxes = Boxes::parse(input);
    for (a, b) in boxes.ordered_by_distance(BOXES) {
        boxes.combine(&a, &b);
    }
    Some(boxes.circuits_by_size().take(3).product())
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut boxes = Boxes::parse(input);
    for (a, b) in boxes.ordered_by_distance(usize::MAX) {
        boxes.combine(&a, &b);
        if boxes.are_all_combined() {
            return Some(a.0 * b.0);
        }
    }
    None
}

type Pos3 = (isize, isize, isize);

struct Boxes {
    boxes: Vec<Pos3>,
    circuits: HashMap<Pos3, usize>,
    next_circuit: usize,
}

impl Boxes {
    fn parse(input: &str) -> Self {
        let boxes = parser!(lines(isize "," isize "," isize))
            .parse(input)
            .expect("Failed to parse");
        Self {
            boxes,
            circuits: HashMap::new(),
            next_circuit: 0,
        }
    }

    fn ordered_by_distance(&self, count: usize) -> Vec<(Pos3, Pos3)> {
        let mut distances = BTreeSet::new();
        for combo in self.boxes.iter().combinations(2) {
            let a = combo[0];
            let b = combo[1];
            let d = distance_sq(&a, &b);
            distances.insert((d, a.clone(), b.clone()));
        }
        distances
            .into_iter()
            .map(|(_, a, b)| (a, b))
            .take(count)
            .collect()
    }

    fn combine(&mut self, a: &Pos3, b: &Pos3) {
        match (self.circuits.get(a), self.circuits.get(b)) {
            (Some(c1), Some(c2)) => {
                if *c1 != *c2 {
                    let cc1 = *c1;
                    let cc2 = *c2;
                    for (_, v) in self.circuits.iter_mut() {
                        if *v == cc2 {
                            *v = cc1;
                        }
                    }
                }
            }
            (Some(c1), None) => {
                self.circuits.insert(b.clone(), *c1);
            }
            (None, Some(c2)) => {
                self.circuits.insert(a.clone(), *c2);
            }
            (None, None) => {
                self.circuits.insert(a.clone(), self.next_circuit);
                self.circuits.insert(b.clone(), self.next_circuit);
                self.next_circuit += 1;
            }
        }
    }

    fn circuits_by_size(&self) -> impl Iterator<Item = usize> {
        let mut groups = BTreeSet::new();
        for (key, grp) in &self
            .circuits
            .iter()
            .sorted_by_key(|&(_, v)| v)
            .chunk_by(|&(_, v)| v)
        {
            groups.insert((grp.count(), key));
        }
        groups.into_iter().rev().map(|(size, _)| size)
    }

    fn are_all_combined(&self) -> bool {
        self.circuits.len() == self.boxes.len()
    }
}

fn distance_sq(a: &Pos3, b: &Pos3) -> isize {
    (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
