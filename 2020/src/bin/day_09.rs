fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file(), 25).unwrap(),
        part_two(read_input_file(), 25)
    );
}

fn read_input_file() -> Vec<usize> {
    let inputs = include_str!("../../resources/day_9.txt")
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    return inputs;
}

fn part_one(inputs: Vec<usize>, preamble_length: usize) -> Option<usize> {
    let mut base_numbers = inputs
        .iter()
        .take(preamble_length)
        .copied()
        .collect::<Vec<usize>>();

    for number in inputs.iter().skip(preamble_length) {
        let current_preamble = base_numbers
            .iter()
            .rev()
            .take(preamble_length)
            .copied()
            .collect::<Vec<usize>>();

        if !is_sum_of_two(&current_preamble, number.clone()) {
            return Some(number.clone());
        }

        base_numbers.push(*number);
    }

    return None;
}

fn is_sum_of_two(numbers: &Vec<usize>, goal: usize) -> bool {
    for i in numbers {
        for j in numbers {
            if i + j == goal {
                return true;
            }
        }
    }

    return false;
}

fn part_two(inputs: Vec<usize>, preamble_length: usize) -> usize {
    let invalid_number = part_one(inputs.clone(), preamble_length).expect("should have a solution");
    let invalid_index = inputs.iter().position(|&x| x == invalid_number).unwrap();

    for i in 0..invalid_index {
        for j in i + 2..invalid_index {
            if inputs[i..=j].iter().sum::<usize>() == invalid_number {
                return inputs[i..=j].iter().min().unwrap() + inputs[i..=j].iter().max().unwrap();
            }
        }
    }

    panic!("no solution found");
}

#[test]
fn day_9_for_part_one() {
    let test_input = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    assert_eq!(part_one(test_input, 5), Some(127));
}

#[test]
fn example_for_partx() {
    let test_input = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    assert_eq!(part_two(test_input, 5), 62);
}
