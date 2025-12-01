advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut code = 50;
    let mut result = 0;
    for line in input.lines() {
        let number: i64 = line[1..].parse().unwrap();
        let number = match &line[..1] {
            "L" => -number,
            "R" => number,
            _ => panic!("Invalid direction"),
        };

        code = (code + number).rem_euclid(100);
        if code == 0 {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut code = 50;
    let mut result = 0;
    for line in input.lines() {
        let number: i64 = line[1..].parse().unwrap();
        let number = match &line[..1] {
            "L" => -number,
            "R" => number,
            _ => panic!("Invalid direction"),
        };

        let new_code = code + number;
        let revolutions = (new_code / 100).abs() as u64;

        let revolutions = if code != 0 && new_code <= 0 {
            revolutions + 1
        } else {
            revolutions
        };

        code = new_code.rem_euclid(100);
        result += revolutions;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
