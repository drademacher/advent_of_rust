fn main() {
    println!("First part: {}\nSecond part: {}", part_one(), part_two());
}

fn read_input_file() -> Vec<u32> {
    return include_str!("day_01.txt")
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
   
        .collect();
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
