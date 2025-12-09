use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .tuple_combinations::<((u64, u64), (u64, u64))>()
        .map(|((x1, y1), (x2, y2))| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    // Idea from https://github.com/blfuentes/AdventOfCode_AllYears/blob/main/AdventOfCode_2025_Go/day09/day09_2.go
    let red_tiles = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .collect_vec();

    let mut result = 0;

    let intersects = |x1: u64, y1: u64, x2: u64, y2: u64| {
        let (min_x, max_x) = (x1.min(x2), x1.max(x2));
        let (min_y, max_y) = (y1.min(y2), y1.max(y2));

        for ((ix1, iy1), (ix2, iy2)) in red_tiles.iter().cloned().circular_tuple_windows() {
            let (i_min_x, i_max_x) = (ix1.min(ix2), ix1.max(ix2));
            let (i_min_y, i_max_y) = (iy1.min(iy2), iy1.max(iy2));
            if min_x < i_max_x && max_x > i_min_x && min_y < i_max_y && max_y > i_min_y {
                return true;
            }
        }
        false
    };

    for ((x1, y1), (x2, y2)) in red_tiles.iter().cloned().tuple_combinations() {
        let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
        if area > result {
            if !intersects(x1, y1, x2, y2) {
                result = area;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
