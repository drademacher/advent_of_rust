#![allow(dead_code, unused_imports, unused_variables)]

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn read_input_file() -> &'static str {
    include_str!("../../resources/day_2.txt")
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|list| !list.is_empty())
        .collect()
}

fn part_one(input: &str) -> i32 {
    let input = parse_input(input);

    input
        .iter()
        .filter(|levels| is_valid_report(levels))
        .count() as i32
}

fn is_valid_report(levels: &[i32]) -> bool {
    let all_decreasing = levels.windows(2).all(|pair| pair[0] > pair[1]);
    let all_increasing = levels.windows(2).all(|pair| pair[0] < pair[1]);
    let distance_within_three = levels.windows(2).all(|pair| (pair[0] - pair[1]).abs() <= 3);

    (all_decreasing || all_increasing) && distance_within_three
}

fn part_two(input: &str) -> i32 {
    let input = parse_input(input);

    input
        .iter()
        .filter(|report| {
            sublists_of_length_n_minus_1(report)
                .iter()
                .any(|sublist| is_valid_report(sublist))
        })
        .count() as i32
}

fn sublists_of_length_n_minus_1(list: &[i32]) -> Vec<Vec<i32>> {
    (0..list.len())
        .map(|i| {
            list.iter()
                .enumerate()
                .filter(|&(index, _)| index != i) // Skip the element at index `i`
                .map(|(_, &value)| value) // Collect the remaining elements
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
    "#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 4);
    }
}
