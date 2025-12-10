extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use pathfinding::directed::bfs::bfs;
use std::rc::Rc;
use z3::{Optimize, SatResult};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let schemas = MachineSchema::parse(input);
    let machines: Vec<_> = schemas.iter().map(|schema| Machine::new(&schema)).collect();
    Some(
        machines
            .into_iter()
            .map(|m| m.fewest_presses_to_start())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let schemas = MachineSchema::parse(input);
    let machines: Vec<_> = schemas.iter().map(|schema| Machine::new(&schema)).collect();
    Some(
        machines
            .into_iter()
            .map(|m| m.fewest_presses_to_configure())
            .sum(),
    )
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct MachineSchema {
    indicator_light_diagram: Vec<bool>,
    button_wiring_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl MachineSchema {
    fn parse(input: &str) -> Vec<Rc<Self>> {
        let nums = parser!(repeat_sep(usize, ","));
        parser!(lines(
            "[" indicator_light_diagram:char_of(".#")+ "] ("
            button_wiring_schematics:repeat_sep(nums, ") (")
            ") {" joltage_requirements:nums "}"
            => Rc::new(Self {
                indicator_light_diagram: indicator_light_diagram.into_iter().map(|i| i == 1).collect(),
                button_wiring_schematics,
                joltage_requirements,
            })
        )).parse(input).expect("Failed to parse")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Machine {
    indicator_lights: Vec<bool>,
    schema: Rc<MachineSchema>,
}

impl Machine {
    fn new(schema: &Rc<MachineSchema>) -> Self {
        let indicator_lights = vec![false; schema.indicator_light_diagram.len()];
        Self {
            indicator_lights,
            schema: schema.clone(),
        }
    }

    fn can_start(&self) -> bool {
        self.indicator_lights == self.schema.indicator_light_diagram
    }

    fn push(&self, button: usize) -> Self {
        assert!(button < self.schema.button_wiring_schematics.len());
        let mut indicator_lights = self.indicator_lights.clone();
        for &i in &self.schema.button_wiring_schematics[button] {
            indicator_lights[i] = !indicator_lights[i];
        }
        Self {
            indicator_lights,
            schema: self.schema.clone(),
        }
    }

    fn fewest_presses_to_start(&self) -> usize {
        bfs(
            self,
            |machine| {
                (0..machine.schema.button_wiring_schematics.len())
                    .map(|button| machine.push(button))
                    .collect::<Vec<_>>()
            },
            |machine| machine.can_start(),
        )
        .map(|path| path.len() - 1)
        .unwrap()
    }

    fn fewest_presses_to_configure(&self) -> usize {
        let opt = Optimize::new();

        let buttons: Vec<_> = (0..self.schema.button_wiring_schematics.len())
            .map(|i| z3::ast::Int::new_const(format!("b{}", i)))
            .collect();

        let zero = z3::ast::Int::from_i64(0);
        for b in &buttons {
            opt.assert(&b.ge(&zero));
        }

        for (counter_idx, &target) in self.schema.joltage_requirements.iter().enumerate() {
            let target_val = z3::ast::Int::from_i64(target as i64);
            let mut sum_terms: Vec<&z3::ast::Int> = vec![];

            for (button_idx, wiring) in self.schema.button_wiring_schematics.iter().enumerate() {
                if wiring.contains(&counter_idx) {
                    sum_terms.push(&buttons[button_idx]);
                }
            }

            let sum = z3::ast::Int::add(&sum_terms);
            opt.assert(&sum.eq(&target_val));
        }

        let all_buttons: Vec<&z3::ast::Int> = buttons.iter().collect();
        let total = z3::ast::Int::add(&all_buttons);
        opt.minimize(&total);

        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model().unwrap();
                let result: i64 = buttons
                    .iter()
                    .map(|b| model.eval(b, true).unwrap().as_i64().unwrap())
                    .sum();
                result as usize
            }
            _ => panic!("No solution found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
