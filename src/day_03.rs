pub fn solve() -> String {
    return format!("First part: {}\nSecond part: {}", part_one(), part_two());
}

fn read_input_file() -> Vec<String> {
    let inputs: Vec<String> = include_str!("day_03.txt")
        .lines()
        .map(|s| String::from(s))
        .collect();
    return inputs;
}

fn part_one() -> usize {
    return trees_for_slope(3, 1);
}

fn part_two() -> usize {
    return trees_for_slope(1, 1)
        * trees_for_slope(3, 1)
        * trees_for_slope(5, 1)
        * trees_for_slope(7, 1)
        * trees_for_slope(1, 2);
}

fn trees_for_slope(right: usize, down: usize) -> usize {
    let inputs = read_input_file();
    let mut result = 0;
    let mut current = 0;

    for i in (0..inputs.len()).step_by(down) {
        let input = inputs.get(i).unwrap();

        if input.chars().nth(current % input.len()).unwrap() == '#' {
            result += 1;
        }

        current += right;
    }

    return result;
}
