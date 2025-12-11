use fxhash::FxHashMap;
use itertools::Itertools;

advent_of_code::solution!(11);

fn count_paths_dag<'a>(
    start: &'a str,
    end: &'a str,
    graph: &FxHashMap<&'a str, Vec<&'a str>>,
    memo: &mut FxHashMap<&'a str, u64>,
) -> u64 {
    if start == end {
        return 1;
    }

    if let Some(&count) = memo.get(start) {
        return count;
    }

    let mut count = 0;
    if let Some(edges) = graph.get(start) {
        for edge in edges {
            count += count_paths_dag(edge, end, graph, memo);
        }
    }

    memo.insert(start, count);
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let mut memo = FxHashMap::default();
    Some(count_paths_dag("you", "out", &graph, &mut memo))
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let svr_to_fft = count_paths_dag("svr", "fft", &graph, &mut FxHashMap::default());
    let svr_to_dac = count_paths_dag("svr", "dac", &graph, &mut FxHashMap::default());
    let fft_to_dac = count_paths_dag("fft", "dac", &graph, &mut FxHashMap::default());
    let dac_to_fft = count_paths_dag("dac", "fft", &graph, &mut FxHashMap::default());
    let fft_to_out = count_paths_dag("fft", "out", &graph, &mut FxHashMap::default());
    let dac_to_out = count_paths_dag("dac", "out", &graph, &mut FxHashMap::default());

    // As this is a DAG, either of this options will be zero,
    // since eihter `fft` is before `dac` or `dac` is before `fft`
    let option1 = svr_to_fft * fft_to_dac * dac_to_out;
    let option2 = svr_to_dac * dac_to_fft * fft_to_out;

    Some(option1 + option2)
}

fn parse_graph(input: &str) -> FxHashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (vert, edges) = line.split_once(": ").unwrap();
            let edges = edges.split(" ").collect_vec();
            (vert, edges)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file_part;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
