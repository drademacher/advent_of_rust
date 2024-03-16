fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn read_input_file() -> Vec<&'static str> {
    return include_str!("../../resources/day_5.txt")
        .lines()
        .collect::<Vec<&str>>();
}

fn part_one(strings: Vec<&'static str>) -> usize {
    return strings
        .iter()
        .filter(|&s| {
            contains_three_vocals(s)
                && cotains_letter_twice_in_a_row(s)
                && does_not_contain_predefined_strings(s)
        })
        .count();
}

fn contains_three_vocals(s: &&str) -> bool {
    s.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
}

fn cotains_letter_twice_in_a_row(s: &&str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..=chars.len() - 2 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }

    return false;
}

fn does_not_contain_predefined_strings(s: &&str) -> bool {
    !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy")
}

fn part_two(strings: Vec<&'static str>) -> usize {
    return strings
        .iter()
        .filter(|&s| {
            contains_pair_of_chars_without_overlapping(s)
                && cotains_letter_twice_with_one_character_distance(s)
        })
        .count();
}

fn contains_pair_of_chars_without_overlapping(s: &&str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..=chars.len() - 2 {
        if s[i + 2..].contains(format!("{}{}", chars[i], chars[i + 1]).as_str()) {
            // println!("found {}{}", chars[i], chars[i+1]);
            return true;
        }
    }

    return false;
}

fn cotains_letter_twice_with_one_character_distance(s: &&str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..=chars.len() - 3 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(vec!["ugknbfddgicrmopn"]), 1);
        assert_eq!(part_one(vec!["aaa"]), 1);
        assert_eq!(part_one(vec!["jchzalrnumimnmhp"]), 0);
        assert_eq!(part_one(vec!["haegwjzuvuyypxyu"]), 0);
        assert_eq!(part_one(vec!["dvszwmarrgswjxmb"]), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(vec!["qjhvhtzxzqqjkmpb"]), 1);
        assert_eq!(part_two(vec!["xxyxx"]), 1);
        assert_eq!(part_two(vec!["uurcxstgmygtbstg"]), 0);
        assert_eq!(part_two(vec!["ieodomkazucvgmuy"]), 0);
    }

    #[test]
    fn test_contains_pair_of_chars_without_overlapping() {
        assert_eq!(contains_pair_of_chars_without_overlapping(&"aaa"), false);
    }
}
