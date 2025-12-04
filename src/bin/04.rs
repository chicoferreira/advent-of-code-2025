use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridValue {
    Empty,
    Roll,
}

impl From<u8> for GridValue {
    fn from(value: u8) -> Self {
        match value {
            b'.' => GridValue::Empty,
            b'@' => GridValue::Roll,
            _ => panic!("Invalid grid value"),
        }
    }
}

struct Grid {
    size: usize,
    data: Vec<GridValue>,
}

impl Grid {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, x: usize, y: usize) -> GridValue {
        self.data[y * self.size + x]
    }

    fn set(&mut self, x: usize, y: usize, value: GridValue) {
        self.data[y * self.size + x] = value;
    }

    fn count_nearby_rolls(&self, x: usize, y: usize) -> u64 {
        let mut rolls = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;
                if ny >= 0 && ny < self.size as i32 && nx >= 0 && nx < self.size as i32 {
                    if self.get(nx as usize, ny as usize) == GridValue::Roll {
                        rolls += 1;
                    }
                }
            }
        }
        rolls
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data = input
            .lines()
            .map(|line| line.as_bytes().into_iter().copied().map(Into::into))
            .flatten()
            .collect::<Vec<GridValue>>();

        let size = data.len().isqrt();

        Ok(Grid { data, size })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_str(input).unwrap();

    let mut result = 0;

    for y in 0..grid.size() {
        for x in 0..grid.size() {
            if grid.get(x, y) != GridValue::Roll {
                continue;
            }

            let rolls = grid.count_nearby_rolls(x, y);

            if rolls < 4 {
                result += 1;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from_str(input).unwrap();

    let mut result = 0;

    loop {
        let mut changed = false;
        for y in 0..grid.size() {
            for x in 0..grid.size() {
                if grid.get(x, y) != GridValue::Roll {
                    continue;
                }

                let rolls = grid.count_nearby_rolls(x, y);

                if rolls < 4 {
                    result += 1;
                    grid.set(x, y, GridValue::Empty);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
