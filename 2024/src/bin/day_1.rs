#![allow(dead_code, unused_imports, unused_variables)]

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn read_input_file() -> &'static str {
    include_str!("../../resources/day_1.txt")
}

fn part_one(input: &'static str) -> i32 {
    let (mut list_left, mut list_right) = transform_input(input);
    list_left.sort();
    list_right.sort();

    list_left
        .iter()
        .zip(list_right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_two(input: &'static str) -> i32 {
    let (list_left, list_right) = transform_input(input);

    list_left
        .iter()
        .map(|number| {
            let occurrences = list_right.iter().filter(|&&x| x == *number).count() as i32;
            number * occurrences
        })
        .sum()
}

fn transform_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            (numbers[0], numbers[1])
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 31);
    }
}
