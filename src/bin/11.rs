extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use topological_sort::TopologicalSort;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Devices::parse(input).count_paths("you", "out"))
}

pub fn part_two(input: &str) -> Option<usize> {
    let devices = Devices::parse(input);
    let ordered = devices.topologically_ordered(&["fft", "dac"]);

    let p1 = devices.count_paths("svr", ordered[0]);
    let p2 = devices.count_paths(ordered[0], ordered[1]);
    let p3 = devices.count_paths(ordered[1], "out");

    Some(p1 * p2 * p3)
}

struct Devices {
    outputs: HashMap<String, HashSet<String>>,
    topological_order: Vec<String>,
    topological_pos: HashMap<String, usize>,
}

impl Devices {
    fn parse(input: &str) -> Self {
        let outputs =
            parser!(hash_map(lines(string(lower+) ": " hash_set(repeat_sep(string(lower+), " ")))))
                .parse(input)
                .expect("Failed to parse");

        let mut ts = TopologicalSort::<String>::new();
        for (d, d_outputs) in &outputs {
            for d2 in d_outputs {
                ts.add_dependency(d.clone(), d2.clone());
            }
        }
        let topological_order: Vec<_> = ts.collect();

        let topological_pos = topological_order
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();

        Self {
            outputs,
            topological_order,
            topological_pos,
        }
    }

    fn topologically_ordered<'a>(&self, devices: &[&'a str]) -> Vec<&'a str> {
        devices
            .into_iter()
            .sorted_by_key(|d| self.topological_pos.get(**d))
            .copied()
            .collect()
    }

    fn count_paths(&self, start: &str, end: &str) -> usize {
        let (s, e) = (self.topological_pos[start], self.topological_pos[end]);
        self.topological_order[s..e]
            .iter()
            .rev()
            .fold(HashMap::from([(end, 1)]), |mut cache, node| {
                let count: usize = self
                    .outputs
                    .get(node)
                    .into_iter()
                    .flatten()
                    .filter_map(|n| cache.get(n.as_str()))
                    .sum();
                if count > 0 {
                    cache.insert(node, count);
                }
                cache
            })
            .get(start)
            .copied()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
