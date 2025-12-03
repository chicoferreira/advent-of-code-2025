advent_of_code::solution!(3);

fn best_joltage_for_2(bank: &str) -> u64 {
    let bank = bank.as_bytes();

    let mut best_first = 0;
    let mut best_second = 0;

    let bank_len = bank.len();

    for first in 0..bank_len - 1 {
        for second in (first + 1)..bank_len {
            let first = bank[first] - b'0';
            let second = bank[second] - b'0';

            if first > best_first {
                best_first = first;
                best_second = second;
            } else if second > best_second {
                best_second = second;
            }
        }
    }
    best_first as u64 * 10u64 + best_second as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(best_joltage_for_2).sum())
}

fn best_joltage(bank: &[u8], digits: u32) -> u64 {
    type Memo = fxhash::FxHashMap<(u32, usize), u64>;
    let mut memo = Memo::default();
    fn helper(bank: &[u8], digits: u32, start_index: usize, memo: &mut Memo) -> u64 {
        if let Some(&result) = memo.get(&(digits, start_index)) {
            return result;
        }

        if bank.len() - start_index < digits as usize {
            return 0;
        }

        let result = match digits {
            0 => 0,
            1 => (bank[start_index..].iter().max().unwrap() - b'0') as u64,
            digits => {
                let skip_solution = helper(bank, digits, start_index + 1, memo);

                let first_digit = (bank[start_index] - b'0') as u64;
                let pow10 = 10u64.pow(digits - 1);
                let take_solution =
                    first_digit * pow10 + helper(bank, digits - 1, start_index + 1, memo);

                take_solution.max(skip_solution)
            }
        };
        memo.insert((digits, start_index), result);
        result
    }

    helper(bank, digits, 0, &mut memo)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|bank| best_joltage(bank.as_bytes(), 12))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_best_joltage() {
        assert_eq!(best_joltage(b"987654321111111", 2), 98);
        assert_eq!(best_joltage(b"811111111111119", 2), 89);
        assert_eq!(best_joltage(b"234234234234278", 2), 78);
        assert_eq!(best_joltage(b"818181911112111", 2), 92);
        assert_eq!(best_joltage(b"987654321111111", 12), 987654321111);
        assert_eq!(best_joltage(b"811111111111119", 12), 811111111119);
        assert_eq!(best_joltage(b"234234234234278", 12), 434234234278);
        assert_eq!(best_joltage(b"818181911112111", 12), 888911112111);
    }
}
