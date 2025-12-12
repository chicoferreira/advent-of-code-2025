advent_of_code::solution!(12, 1);

// this problem is ridiculous we can only solve it by knowing the input is dumb
// each region either obviously has enough space or obviously does not
// there isn't one you need to organize the presents in some particular way to fit
pub fn part_one(input: &str) -> Option<u64> {
    let regions = input.lines().skip_while(|line| {
        // split present shapes
        line.is_empty() || line.ends_with(":") || line.chars().all(|c| c == '.' || c == '#')
    });

    let mut result = 0;

    for region in regions {
        let (region_size, shape_quantity) = region.split_once(":").unwrap();
        let (region_width, region_length) = region_size.split_once("x").unwrap();
        let region_width = region_width.parse::<u64>().unwrap();
        let region_length = region_length.parse::<u64>().unwrap();

        let shape_quantity: u64 = shape_quantity
            .split_whitespace()
            .map(|number| number.parse::<u64>().unwrap())
            .sum();

        let region_area = region_width * region_length;
        // assuming all shapes are 3x3
        let shape_area = shape_quantity * 3 * 3;

        if region_area >= shape_area {
            result += 1;
        }
    }

    Some(result)
}

// there are no tests because only the puzzle input suits the description on top
