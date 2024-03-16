// use std::collections::HashSet;
// use std::iter::FromIterator;

// pub fn solve() -> String {
//     return format!(
//         "First part: {}\nSecond part: {}",
//         part_one(read_input_file()),
//         part_two(read_input_file())
//     );
// }

// fn read_input_file() -> Vec<String> {
//     let inputs: Vec<String> = include_str!("day_06.txt")
//         .lines()
//         .map(|s| String::from(s))
//         .collect();
//     return inputs;
// }

// fn part_one(inputs: Vec<String>) -> usize {
//     let mut result = 0;
//     let mut answers_from_any: HashSet<char> = HashSet::new();

//     for input in inputs {
//         if input == "" {
//             result += answers_from_any.len();
//             answers_from_any = HashSet::new();
//             continue;
//         }

//         answers_from_any = answers_from_any
//             .union(&input.chars().collect())
//             .cloned()
//             .collect();
//     }
//     result += answers_from_any.len();

//     return result;
// }

// fn part_two(inputs: Vec<String>) -> usize {
//     let mut result = 0;
//     let mut answers_from_all = HashSet::new();
//     let mut start_new_group = true;

//     for input in inputs {
//         if input == "" {
//             result += answers_from_all.len();
//             start_new_group = true;
//         } else if start_new_group {
//             answers_from_all = HashSet::from_iter(input.chars());
//             start_new_group = false;
//         } else {
//             let new_answers = &HashSet::from_iter(input.chars());
//             answers_from_all = answers_from_all
//                 .intersection(new_answers)
//                 .cloned()
//                 .collect();
//         }
//     }
//     result += answers_from_all.len();

//     return result;
// }

// #[test]
// fn example_for_part_one() {
//     let test_inputs = vec![
//         "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
//     ]
//     .iter()
//     .map(|&s| String::from(s))
//     .collect::<Vec<_>>();
//     assert_eq!(part_one(test_inputs), 11);
// }

// #[test]
// fn example_for_part_two() {
//     let test_inputs = vec![
//         "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
//     ]
//     .iter()
//     .map(|&s| String::from(s))
//     .collect::<Vec<_>>();
//     assert_eq!(part_two(test_inputs), 6);
// }
