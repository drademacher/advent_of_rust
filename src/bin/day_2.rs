use std::{num::ParseIntError, str::FromStr};

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

#[derive(Debug, Eq, PartialEq)]
struct Box {
    length: u32,
    width: u32,
    height: u32,
}

impl FromStr for Box {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split("x");
        let cuboid = Box {
            length: numbers.next().unwrap().parse::<u32>()?,
            width: numbers.next().unwrap().parse::<u32>()?,
            height: numbers.next().unwrap().parse::<u32>()?,
        };

        Ok(cuboid)
    }
}

fn read_input_file() -> Vec<Box> {
    return include_str!("../../resources/2015/day_2.txt")
        .lines()
        .map(|s| s.parse::<Box>().unwrap())
        .collect();
}

fn part_one(input: Vec<Box>) -> u32 {
    return input
        .iter()
        .map(|b| {
            2 * b.length * b.width
                + 2 * b.width * b.height
                + 2 * b.height * b.length
                + vec![b.length * b.width, b.width * b.height, b.height * b.length]
                    .iter()
                    .min()
                    .unwrap()
        })
        .sum();
}

fn part_two(input: Vec<Box>) -> u32 {
    return input
        .iter()
        .map(|b| {
            b.length * b.width * b.height
                + 2 * vec![b.length + b.width, b.width + b.height, b.height + b.length]
                    .iter()
                    .min()
                    .unwrap()
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_for_box() {
        assert_eq!(
            "27x16x1".parse::<Box>().unwrap(),
            Box {
                length: 27,
                width: 16,
                height: 1
            }
        );
    }

    #[test]
    fn test_part_one() {
        let first = Box {
            length: 2,
            width: 3,
            height: 4,
        };
        let second = Box {
            length: 1,
            width: 1,
            height: 10,
        };

        assert_eq!(part_one(vec![first]), 58);
        assert_eq!(part_one(vec![second]), 43);
    }

    #[test]
    fn test_part_two() {
        let first = Box {
            length: 2,
            width: 3,
            height: 4,
        };
        let second = Box {
            length: 1,
            width: 1,
            height: 10,
        };

        assert_eq!(part_two(vec![first]), 34);
        assert_eq!(part_two(vec![second]), 14);
    }
}
