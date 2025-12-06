use std::{panic, str::FromStr};

advent_of_code::solution!(6);

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(format!("invalid operator: {}", s)),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input
        .lines()
        .map(|line| line.split_whitespace())
        .collect::<Vec<_>>();

    let Some((operators_line, numbers_lines)) = lines.split_last_mut() else {
        panic!("invalid input")
    };

    let mut result = 0;

    loop {
        let Some(operator) = operators_line.next() else {
            break;
        };

        let operator = operator.parse().unwrap();

        let mut line_result = match operator {
            Operator::Add => 0,
            Operator::Multiply => 1,
        };

        for line in &mut *numbers_lines {
            let number: u64 = line.next().unwrap().parse().unwrap();
            match operator {
                Operator::Add => line_result += number,
                Operator::Multiply => line_result *= number,
            }
        }
        result += line_result;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(str::chars).collect::<Vec<_>>();

    let Some((operators_line, numbers_lines)) = lines.split_last_mut() else {
        panic!("invalid input")
    };

    let mut result = 0;

    let mut current_problem = 0;
    let mut current_operator = Operator::Add;

    loop {
        let Some(operator) = operators_line.next() else {
            break;
        };

        match operator {
            '+' => {
                result += current_problem;
                current_operator = Operator::Add;
                current_problem = 0;
            }
            '*' => {
                result += current_problem;
                current_operator = Operator::Multiply;
                current_problem = 1;
            }
            ' ' => {}
            _ => panic!("invalid operator"),
        };

        let mut number = 0;

        for line in &mut *numbers_lines {
            let char = line.next().unwrap();
            if let Some(digit) = char.to_digit(10) {
                number = number * 10 + digit as u64;
            }
        }

        // check if number has changed (we are not in the last index of the column of this number)
        if number != 0 {
            match current_operator {
                Operator::Add => current_problem += number,
                Operator::Multiply => current_problem *= number,
            }
        }
    }

    result += current_problem;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
