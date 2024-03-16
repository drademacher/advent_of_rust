fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn read_input_file() -> &'static str {
    return include_str!("../../resources/day_1.txt");
}

fn part_one(input: &'static str) -> i32 {
    let mut floor = 0;

    for char in input.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    return floor;
}

fn part_two(input: &'static str) -> u32 {
    let mut floor = 0;
    let mut position = 0;

    for char in input.chars() {
        position += 1;
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        if floor == -1 {
            break;
        }
    }

    return position;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), 0);
        assert_eq!(part_one("()()"), 0);
        assert_eq!(part_one("((("), 3);
        assert_eq!(part_one("(()(()("), 3);
        assert_eq!(part_one("))((((("), 3);
        assert_eq!(part_one("())"), -1);
        assert_eq!(part_one("))("), -1);
        assert_eq!(part_one(")))"), -3);
        assert_eq!(part_one(")())())"), -3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(")"), 1);
        assert_eq!(part_two("()())"), 5);
    }
}
