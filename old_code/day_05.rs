// use std::convert::TryInto;
// use std::vec::Vec;

// pub fn solve() -> String {
//     return format!("First part: {}\nSecond part: {}", part_one(), part_two());
// }

// struct BoardPass {
//     _row: usize,
//     _column: usize,
//     seat_id: usize,
// }

// impl BoardPass {
//     fn new(encoded: &str) -> Self {
//         let _row = encoded[..7]
//             .char_indices()
//             .filter(|&(_, c)| c == 'B')
//             .map(|(i, _)| 2usize.pow((6 - i).try_into().unwrap()))
//             .sum::<usize>();
//         let _column = encoded[7..]
//             .char_indices()
//             .filter(|&(_, c)| c == 'R')
//             .map(|(i, _)| 2usize.pow((2 - i).try_into().unwrap()))
//             .sum::<usize>();
//         let seat_id = _row * 8 + _column;

//         return BoardPass {
//             _row,
//             _column,
//             seat_id,
//         };
//     }
// }

// fn read_input_file() -> Vec<String> {
//     let inputs: Vec<String> = include_str!("day_05.txt")
//         .lines()
//         .map(|s| String::from(s))
//         .collect();
//     return inputs;
// }

// fn part_one() -> usize {
//     let inputs = read_input_file();
//     return inputs
//         .iter()
//         .map(|s| BoardPass::new(s))
//         .map(|pass| pass.seat_id)
//         .max()
//         .unwrap();
// }

// fn part_two() -> usize {
//     let inputs = read_input_file();
//     let seat_ids = inputs
//         .iter()
//         .map(|s| BoardPass::new(s))
//         .map(|pass| pass.seat_id)
//         .collect::<Vec<usize>>();

//     let sum_start = seat_ids.iter().min().unwrap();
//     let sum_end = seat_ids.iter().max().unwrap();
//     let full_sum = sum_end * (sum_end + 1) / 2;
//     let sum_below_start = (sum_start - 1) * sum_start / 2;

//     return full_sum - sum_below_start - seat_ids.iter().sum::<usize>();
// }

// #[test]
// fn board_pass_1() {
//     let board_pass = BoardPass::new("BFFFBBFRRR");
//     assert_eq!(board_pass._row, 70);
//     assert_eq!(board_pass._column, 7);
//     assert_eq!(board_pass.seat_id, 567);
// }

// #[test]
// fn board_pass_2() {
//     let board_pass = BoardPass::new("FFFBBBFRRR");
//     assert_eq!(board_pass._row, 14);
//     assert_eq!(board_pass._column, 7);
//     assert_eq!(board_pass.seat_id, 119);
// }

// #[test]
// fn board_pass_3() {
//     let board_pass = BoardPass::new("BBFFBBFRLL");
//     assert_eq!(board_pass._row, 102);
//     assert_eq!(board_pass._column, 4);
//     assert_eq!(board_pass.seat_id, 820);
// }
