use fxhash::FxHashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let mut grid_iter = grid.into_iter();

    let mut beams = grid_iter
        .next()
        .unwrap()
        .into_iter()
        .map(|c| match c {
            b'S' => true,
            b'.' => false,
            _ => unreachable!("invalid character"),
        })
        .collect::<Vec<bool>>();

    let mut result = 0;

    while let Some(line) = grid_iter.next() {
        for index in 0..beams.len() {
            let ch = line[index];
            let beam = beams[index];
            if ch == b'^' && beam {
                result += 1;
                beams[index] = false;
                if index > 0 {
                    beams[index - 1] = true;
                }
                if index < beams.len() - 1 {
                    beams[index + 1] = true;
                }
            }
        }
    }

    Some(result)
}

fn ways_down(
    index: usize,
    depth: usize,
    grid: &[&[u8]],
    memo: &mut FxHashMap<(usize, usize), u64>,
) -> u64 {
    if depth >= grid.len() {
        return 1;
    }

    if let Some(&cached) = memo.get(&(index, depth)) {
        return cached;
    }

    let line = grid[depth];
    let ch = line[index];

    let result = if ch == b'^' {
        let mut total = 0;

        if index > 0 {
            total += ways_down(index - 1, depth + 1, grid, memo);
        }

        if index + 1 < line.len() {
            total += ways_down(index + 1, depth + 1, grid, memo);
        }

        total
    } else {
        ways_down(index, depth + 1, grid, memo)
    };

    memo.insert((index, depth), result);
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();

    let first_beam_index = grid[0].iter().position(|ch| *ch == b'S').unwrap();

    let mut memo = FxHashMap::default();
    let result = ways_down(first_beam_index, 0, &grid, &mut memo);

    Some(result)
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
