use std::collections::HashSet;

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point(i32, i32);

fn read_input_file() -> &'static str {
    return include_str!("../../resources/2015/day_3.txt");
}

fn part_one(input: &'static str) -> usize {
    let mut visited = HashSet::new();

    let mut current = Point(0, 0);
    visited.insert(current);

    for direction in input.chars() {
        match direction {
            '>' => current.0 += 1,
            '<' => current.0 -= 1,
            '^' => current.1 += 1,
            'v' => current.1 -= 1,
            _ => panic!("invalid characters in input {}", direction),
        }

        visited.insert(current);
    }

    return visited.len();
}

fn part_two(input: &'static str) -> usize {
    let mut visited_by_santa = HashSet::new();
    let mut visited_by_robo_santa = HashSet::new();

    let mut santa = Point(0, 0);
    let mut robo_santa = Point(0, 0);
    visited_by_santa.insert(santa);
    visited_by_robo_santa.insert(robo_santa);

    for (index, direction) in input.chars().enumerate() {
        if index % 2 == 0 {
            match direction {
                '>' => santa.0 += 1,
                '<' => santa.0 -= 1,
                '^' => santa.1 += 1,
                'v' => santa.1 -= 1,
                _ => panic!("invalid characters in input {}", direction),
            }

            visited_by_santa.insert(santa);
        } else {
            match direction {
                '>' => robo_santa.0 += 1,
                '<' => robo_santa.0 -= 1,
                '^' => robo_santa.1 += 1,
                'v' => robo_santa.1 -= 1,
                _ => panic!("invalid characters in input {}", direction),
            }

            visited_by_robo_santa.insert(robo_santa);
        }
    }

    let mut visited = visited_by_santa;
    visited.extend(visited_by_robo_santa);

    return visited.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(">"), 2);
        assert_eq!(part_one("^>v<"), 4);
        assert_eq!(part_one("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v"), 3);
        assert_eq!(part_two("^>v<"), 3);
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }
}
