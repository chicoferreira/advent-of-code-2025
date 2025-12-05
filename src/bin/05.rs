use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            // move to parsing ids
            break;
        }
        let (start, end) = line.split_once("-").unwrap();
        let (start, end) = (start.parse().unwrap(), end.parse().unwrap());
        let range = RangeInclusive::new(start, end);
        ranges.push(range);
    }

    let mut result = 0;

    while let Some(line) = lines.next() {
        let id = line.parse().unwrap();
        for range in &ranges {
            if range.contains(&id) {
                result += 1;
                break;
            }
        }
    }

    Some(result)
}

struct RangeSet {
    ranges: Vec<(u64, u64)>,
}

impl RangeSet {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn insert(&mut self, start: u64, end: u64) {
        // Find first range that might overlap/adjacent
        let i = self.ranges.partition_point(|p| p.1 + 1 < start);

        // No overlap/adjacent
        if i == self.ranges.len() || self.ranges[i].0 > end + 1 {
            self.ranges.insert(i, (start, end));
            return;
        }

        // Merge with current range
        let start = start.min(self.ranges[i].0);
        let end = end.max(self.ranges[i].1);

        // Absorb all subsequent ranges that still overlap/adjacent
        let offset = self.ranges[i + 1..].partition_point(|p| p.0 <= end + 1);
        let j = i + 1 + offset;
        let end = end.max(self.ranges[j - 1].1);

        self.ranges.splice(i..j, [(start, end)]);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut ranges = RangeSet::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once("-").unwrap();
        let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

        ranges.insert(start, end);
    }

    let mut total = 0;
    for (start, end) in ranges.ranges {
        total += end - start + 1;
    }

    Some(total)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_range_set() {
        let mut set = RangeSet::new();
        set.insert(1, 5);
        assert_eq!(set.ranges, vec![(1, 5)]);
        set.insert(3, 7);
        assert_eq!(set.ranges, vec![(1, 7)]);
        set.insert(10, 15);
        assert_eq!(set.ranges, vec![(1, 7), (10, 15)]);
        set.insert(8, 9);
        assert_eq!(set.ranges, vec![(1, 15)]);
        set.insert(20, 25);
        assert_eq!(set.ranges, vec![(1, 15), (20, 25)]);
        set.insert(30, 40);
        assert_eq!(set.ranges, vec![(1, 15), (20, 25), (30, 40)]);
        set.insert(1, 50);
        assert_eq!(set.ranges, vec![(1, 50)]);
    }
}
