#![allow(dead_code, unused_imports, unused_variables)]

use regex::Regex;
use serde_json::Value;

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn read_input_file() -> &'static str {
    return include_str!("../../resources/day_12.txt");
}

fn part_one(input: &'static str) -> i32 {
    let regex = Regex::new(r"(\d|-)+").unwrap();

    // let captures = regex.captures_iter(input);
    // println!("{:?}", captures);

    return regex
        .find_iter(input)
        .filter_map(|digits| digits.as_str().parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .iter()
        .sum();
}

fn part_two(input: &'static str) -> u32 {
    let v: Value = serde_json::from_str(input).unwrap();

    println!("{:#?}", v);

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // assert_eq!(part_one(""), );
    }

    #[test]
    fn test_part_two() {
        // assert_eq!(part_two(""), );
    }
}
