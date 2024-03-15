fn main() {
    println!("First part: {}\nSecond part: {}", part_one(), part_two());
}

fn read_input_file() -> &'static str {
    return include_str!("../../resources/2015/day_1.txt");
}

fn part_one() -> u32 {
    read_input_file();
    return 0;
}

fn part_two() -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two();
        assert_eq!(result, 0);
    }
}
