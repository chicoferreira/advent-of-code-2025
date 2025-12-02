advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;

    for range in input.split(",") {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.trim().parse::<u64>().unwrap();

        for number in start..=end {
            let number_size = number.ilog10() + 1;
            if number_size % 2 != 0 {
                continue;
            }

            let divisor = number_size / 2;
            let left = number / 10u64.pow(divisor);
            let right = number % 10u64.pow(divisor);

            if left == right {
                result += number;
            }
        }
    }

    Some(result)
}

fn get_number_slice(number: u64, start: u32, end: u32) -> u64 {
    let number_size = number.ilog10() + 1;
    number / 10u64.pow(number_size - end - 1) % 10u64.pow(end - start + 1)
}

fn is_id_invalid_part_two(number: u64) -> bool {
    let number_size = number.ilog10() + 1;

    'a: for number_slice_size in 1..=number_size / 2 {
        if number_size % number_slice_size != 0 {
            continue;
        }
        let first_slice = get_number_slice(number, 0, number_slice_size - 1);
        for slice_index in (number_slice_size..number_size).step_by(number_slice_size as usize) {
            let slice = get_number_slice(number, slice_index, slice_index + number_slice_size - 1);

            if first_slice != slice {
                continue 'a;
            }
        }

        return true;
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;

    for range in input.split(",") {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.trim().parse::<u64>().unwrap();

        for number in start..=end {
            if is_id_invalid_part_two(number) {
                result += number;
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_number_slice() {
        assert_eq!(get_number_slice(1010, 1, 2), 01);
        assert_eq!(get_number_slice(587256255, 2, 3), 72);
        assert_eq!(get_number_slice(120000, 0, 1), 12);
    }

    #[test]
    fn is_number_invalid_part_two() {
        assert_eq!(is_id_invalid_part_two(11), true);
        assert_eq!(is_id_invalid_part_two(22), true);
        assert_eq!(is_id_invalid_part_two(99), true);
        assert_eq!(is_id_invalid_part_two(111), true);
        assert_eq!(is_id_invalid_part_two(999), true);
        assert_eq!(is_id_invalid_part_two(1010), true);
        assert_eq!(is_id_invalid_part_two(1188511885), true);
        assert_eq!(is_id_invalid_part_two(222222), true);
        assert_eq!(is_id_invalid_part_two(446446), true);
        assert_eq!(is_id_invalid_part_two(38593859), true);
        assert_eq!(is_id_invalid_part_two(565656), true);
        assert_eq!(is_id_invalid_part_two(824824824), true);
        assert_eq!(is_id_invalid_part_two(2121212121), true);
        assert_eq!(is_id_invalid_part_two(2121212118), false);
    }
}
