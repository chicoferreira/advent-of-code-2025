use fxhash::FxHashMap;
use itertools::Itertools;

advent_of_code::solution!(8);

type Coord = (u32, u32, u32);

fn distance(a: Coord, b: Coord) -> f32 {
    ((a.0 as f32 - b.0 as f32).powi(2)
        + (a.1 as f32 - b.1 as f32).powi(2)
        + (a.2 as f32 - b.2 as f32).powi(2))
    .sqrt()
}

fn part_one_with_pairs(input: &str, pairs_n: usize) -> Option<u64> {
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            line.splitn(3, ",")
                .map(|part| part.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut pairs: Vec<(Coord, Coord)> = coords.into_iter().tuple_combinations().collect();

    let (top_pairs, _, _) = pairs.select_nth_unstable_by(pairs_n, |a, b| {
        distance(a.0, a.1).partial_cmp(&distance(b.0, b.1)).unwrap()
    });

    let mut current_circuit_index = 0;
    let mut circuits = fxhash::FxHashMap::default();

    for (pair_a, pair_b) in top_pairs {
        let a_circuit = circuits.get(&pair_a).cloned();
        let b_circuit = circuits.get(&pair_b).cloned();

        match (a_circuit, b_circuit) {
            (None, None) => {
                current_circuit_index += 1;
                circuits.insert(pair_a, current_circuit_index);
                circuits.insert(pair_b, current_circuit_index);
            }
            (None, Some(b_circuit)) => {
                circuits.insert(pair_a, b_circuit);
            }
            (Some(a_circuit), None) => {
                circuits.insert(pair_b, a_circuit);
            }
            (Some(a_circuit), Some(b_circuit)) if a_circuit != b_circuit => {
                circuits
                    .iter_mut()
                    .filter(|(_, value)| **value == b_circuit)
                    .for_each(|(_, value)| *value = a_circuit);
            }
            (Some(_), Some(_)) => {}
        }
    }

    let mut circuit_sizes = FxHashMap::default();
    for (_, circuit) in circuits {
        *circuit_sizes.entry(circuit).or_insert(0) += 1;
    }

    let mut sizes: Vec<u64> = circuit_sizes.values().cloned().collect();
    sizes.sort_unstable();
    sizes.reverse();

    Some(sizes[0..3].iter().product())
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_with_pairs(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            line.splitn(3, ",")
                .map(|part| part.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    // insight: the last to connect will be the coord furthest away from the closest coord

    let mut best_x_multiplied = 0u64;
    let mut highest_distance = 0.0f32;

    for coord in coords.iter() {
        let closest_coord = coords
            .iter()
            .filter(|&c| c != coord)
            .min_by(|&c1, &c2| distance(*coord, *c1).total_cmp(&distance(*coord, *c2)))
            .unwrap();

        let distance = distance(*coord, *closest_coord);
        if distance > highest_distance {
            best_x_multiplied = coord.0 as u64 * closest_coord.0 as u64;
            highest_distance = distance;
        }
    }

    Some(best_x_multiplied)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_with_pairs(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
