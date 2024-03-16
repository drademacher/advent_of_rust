use std::collections::HashMap;

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(vec![0, 1, 4, 13, 15, 12, 16]),
        part_two(vec![0, 1, 4, 13, 15, 12, 16])
    );
}

fn part_one(inputs: Vec<usize>) -> usize {
    return memory_game(inputs, 2020);
}

fn part_two(inputs: Vec<usize>) -> usize {
    return memory_game(inputs, 30000000);
}

fn memory_game(inputs: Vec<usize>, goal_number: usize) -> usize {
    let mut last_seen: HashMap<usize, usize> = HashMap::new();

    inputs[..inputs.len() - 1]
        .iter()
        .enumerate()
        .for_each(|(index, input)| {
            last_seen.insert(*input, index + 1);
        });

    let mut current = inputs.iter().last().unwrap().clone();

    for turn in inputs.len() + 1..goal_number + 1 {
        let next = match last_seen.get(&current.clone()) {
            Some(second_last_turn) => (turn - 1) - second_last_turn,
            None => 0,
        };

        last_seen.insert(current.clone(), turn - 1);

        current = next;
    }

    return current;
}

#[test]
fn day_15_examples_one() {
    assert_eq!(part_one(vec![0, 3, 6]), 436);
    assert_eq!(part_one(vec![1, 3, 2]), 1);
    assert_eq!(part_one(vec![2, 1, 3]), 10);
    assert_eq!(part_one(vec![1, 2, 3]), 27);
    assert_eq!(part_one(vec![2, 3, 1]), 78);
    assert_eq!(part_one(vec![3, 2, 1]), 438);
    assert_eq!(part_one(vec![3, 1, 2]), 1836);
}

// takes very long amount of time
#[test]
#[ignore]
fn day_15_examples_two() {
    assert_eq!(part_two(vec![0, 3, 6]), 175594);
    assert_eq!(part_two(vec![1, 3, 2]), 2578);
    assert_eq!(part_two(vec![2, 1, 3]), 3544142);
    assert_eq!(part_two(vec![1, 2, 3]), 261214);
    assert_eq!(part_two(vec![2, 3, 1]), 6895259);
    assert_eq!(part_two(vec![3, 2, 1]), 18);
    assert_eq!(part_two(vec![3, 1, 2]), 362);
}
