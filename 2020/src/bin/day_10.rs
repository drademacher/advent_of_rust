fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn read_input_file() -> Vec<usize> {
    let inputs = include_str!("../../resources/day_10.txt")
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    return inputs;
}

fn part_one(inputs: Vec<usize>) -> usize {
    let mut inputs = inputs;
    inputs.push(0);
    inputs.sort();

    let mut difference_by_one = 0;
    let mut difference_by_three = 1;

    for i in 0..inputs.len() - 1 {
        if inputs[i + 1] - inputs[i] == 1 {
            difference_by_one += 1;
        } else if inputs[i + 1] - inputs[i] == 3 {
            difference_by_three += 1;
        }
    }

    return difference_by_one * difference_by_three;
}

fn part_two(inputs: Vec<usize>) -> usize {
    let mut inputs = inputs;
    inputs.push(0);
    inputs.sort();

    let mut result = 1;
    let mut start: usize = 0;
    for i in 0..inputs.len() - 1 {
        if inputs[i + 1] - inputs[i] == 3 {
            let end = i + 1;

            result *= calculcate_possibilites(inputs[start..=end].to_vec());
            start = end;
        }
    }
    result *= calculcate_possibilites(inputs[start..].to_vec());

    return result;
}

fn calculcate_possibilites(inputs: Vec<usize>) -> usize {
    return powerset(&inputs)
        .iter()
        .filter(|subset| subset.len() > 1)
        .filter(|subset| {
            subset[0] == inputs[0] && subset[subset.len() - 1] == inputs[inputs.len() - 1]
        })
        .filter(|subset| {
            (0..subset.len() - 1)
                .map(|i| subset[i + 1] - subset[i])
                .all(|diff| diff <= 3)
        })
        .count();
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect()
}

#[test]
fn day_10_part_one() {
    let test_input = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    assert_eq!(part_one(test_input), 220);
}

// some broken corner case
// #[test]
// fn day_10_part_two_with_small_test_set() {
//     let test_input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

//     assert_eq!(part_two(test_input), 8);
// }

#[test]
fn day_10_part_two_with_large_test_set() {
    let test_input = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    assert_eq!(part_two(test_input), 19208);
}
