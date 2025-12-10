use std::collections::VecDeque;

use fxhash::FxHashSet;
use itertools::Itertools;
use microlp::OptimizationDirection;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(10);

struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

fn fewest_presses_p1(machine: Machine) -> u64 {
    let mut stack = VecDeque::new();

    let lights_len = machine.target_lights.len();
    let start_lights = vec![false; lights_len];
    for button_index in 0..machine.buttons.len() {
        stack.push_front((start_lights.clone(), button_index, 0));
    }

    let mut visited = FxHashSet::default();
    visited.insert(start_lights.clone());

    while let Some((mut lights, button_index, depth)) = stack.pop_back() {
        let button = &machine.buttons[button_index];
        for &light_index in button {
            lights[light_index] = !lights[light_index];
        }
        if lights == machine.target_lights {
            return depth + 1;
        }

        if visited.contains(&lights) {
            continue;
        }
        visited.insert(lights.clone());

        for button_index in 0..machine.buttons.len() {
            stack.push_front((lights.clone(), button_index, depth + 1));
        }
    }

    panic!("No solution found")
}

// takes too long
fn _fewest_presses_p2_naive(machine: Machine) -> u64 {
    let mut stack = VecDeque::new();

    let joltages_len = machine.joltages.len();
    let start_joltages = vec![0u64; joltages_len];
    for button_index in 0..machine.buttons.len() {
        stack.push_front((start_joltages.clone(), button_index, 0));
    }

    let mut visited = FxHashSet::default();
    visited.insert(start_joltages.clone());

    while let Some((mut joltages, button_index, depth)) = stack.pop_back() {
        let button = &machine.buttons[button_index];
        for &joltage_index in button {
            joltages[joltage_index] += 1;
        }
        if joltages == machine.joltages {
            return depth + 1;
        }

        if visited.contains(&joltages) {
            continue;
        }
        visited.insert(joltages.clone());

        for button_index in 0..machine.buttons.len() {
            stack.push_front((joltages.clone(), button_index, depth + 1));
        }
    }

    panic!("No solution found")
}

// For the input `(3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}`
// The problem stands as follows (with ni being the number of presses of button i)
//
// 3 = n4 + n5
// 5 = n1 + n5
// 4 = n2 + n3 + n4
// 7 = n0 + n1 + n3
//
// Minimize(n0 + n1 + n2 + n3 + n4 + n5)
fn fewest_presses_p2_lp(machine: Machine) -> u64 {
    let mut lp_problem = microlp::Problem::new(OptimizationDirection::Minimize);

    let variables = (0..machine.buttons.len())
        .into_iter()
        .map(|_| lp_problem.add_integer_var(1.0, (0, i32::MAX)))
        .collect_vec();

    for (joltage_index, joltage) in machine.joltages.into_iter().enumerate() {
        let expr = machine
            .buttons
            .iter()
            .enumerate()
            .filter(|(_, button)| button.contains(&joltage_index))
            .map(|(i, _)| variables[i].clone())
            .map(|var| (var, 1.0))
            .collect_vec();

        lp_problem.add_constraint(&expr, microlp::ComparisonOp::Eq, joltage as f64);
    }

    let solution = lp_problem.solve().unwrap();
    variables
        .into_iter()
        .map(|var| solution[var].round() as u64)
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = input.lines().map(parse_machine).collect_vec();
    Some(machines.into_par_iter().map(fewest_presses_p1).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = input.lines().map(parse_machine).collect_vec();
    Some(machines.into_par_iter().map(fewest_presses_p2_lp).sum())
}

fn parse_machine(line: &str) -> Machine {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let n = parts.len();

    let target_lights = parts[0]
        .bytes()
        .filter(|&b| b == b'.' || b == b'#')
        .map(|b| b == b'#')
        .collect();

    let joltages = parts[n - 1]
        .trim_matches(['{', '}'])
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let buttons = parts[1..n - 1]
        .iter()
        .map(|&s| {
            s.trim_matches(['(', ')'])
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    Machine {
        target_lights,
        buttons,
        joltages,
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
