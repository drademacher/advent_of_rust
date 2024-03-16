use regex::Regex;

fn main() {
    println!("First part: {}\nSecond part: {}", part_one(), part_two());
}

#[derive(Debug)]
struct SingleInput {
    fst_number: usize,
    snd_number: usize,
    character: char,
    password: &'static str,
}

fn read_input_file() -> Vec<SingleInput> {
    let regex = Regex::new(r"(\d+)-(\d+) (\w+): (\w+)").unwrap();
    let inputs: Vec<SingleInput> = include_str!("../../resources/day_2.txt")
        .lines()
        .map(|s| regex.captures_iter(s).next().unwrap())
        .map(|cap| SingleInput {
            fst_number: cap[1].parse().unwrap(),
            snd_number: cap[2].parse().unwrap(),
            character: cap.get(3).unwrap().as_str().chars().next().unwrap(),
            password: cap.get(4).unwrap().as_str(),
        })
        .collect();
    return inputs;
}

fn part_one() -> usize {
    let inputs = read_input_file();
    let mut result = 0;

    for input in inputs {
        let count_character = input
            .password
            .chars()
            .filter(|&c| c == input.character)
            .count();
        if input.fst_number <= count_character && count_character <= input.snd_number {
            result += 1;
        }
    }

    return result;
}

fn part_two() -> usize {
    let inputs = read_input_file();
    let mut result = 0;

    for input in inputs {
        let is_first_char_matching =
            input.password.chars().nth(input.fst_number - 1).unwrap() == input.character;
        let is_second_char_matching =
            input.password.chars().nth(input.snd_number - 1).unwrap() == input.character;

        if is_first_char_matching != is_second_char_matching {
            result += 1;
        }
    }

    return result;
}
