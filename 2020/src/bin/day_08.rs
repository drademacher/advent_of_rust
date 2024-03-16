// use std::collections::HashSet;

// pub fn solve() -> String {
//     return format!(
//         "First part: {}\nSecond part: {}",
//         part_one(parse_inputs(read_input_file())),
//         part_two(parse_inputs(read_input_file()))
//     );
// }

// fn read_input_file() -> Vec<String> {
//     let inputs = include_str!("day_08.txt")
//         .lines()
//         .map(|s| String::from(s))
//         .collect::<Vec<_>>();
//     return inputs;
// }

// fn parse_inputs(inputs: Vec<String>) -> Vec<Instruction> {
//     return inputs.into_iter().map(|s| Instruction::from(s)).collect();
// }

// fn part_one(inputs: Vec<Instruction>) -> isize {
//     if let SimulationResult::Loop(accumulator) = simulate(&inputs) {
//         return accumulator;
//     }

//     panic!("no solution found");
// }

// fn part_two(inputs: Vec<Instruction>) -> isize {
//     let mut inputs = inputs;

//     for i in 0..inputs.len() {
//         inputs[i] = toggle_instruction(inputs[i]);

//         if let SimulationResult::Terminated(accumulator) = simulate(&inputs) {
//             return accumulator;
//         }

//         inputs[i] = toggle_instruction(inputs[i]);
//     }
//     panic!("no solution found");
// }

// fn toggle_instruction(instruction: Instruction) -> Instruction {
//     return match instruction {
//         Instruction::Nop(number) => Instruction::Jump(number),
//         Instruction::Jump(number) => Instruction::Nop(number),
//         Instruction::Accumulator(number) => Instruction::Accumulator(number),
//     };
// }

// #[derive(Debug, Clone, Copy)]
// enum Instruction {
//     Nop(isize),
//     Accumulator(isize),
//     Jump(isize),
// }

// impl Instruction {
//     fn from(raw_value: String) -> Self {
//         let split = raw_value.split(&" ").collect::<Vec<&str>>();
//         let number = split[1].parse::<isize>().unwrap();

//         match split[0] {
//             "nop" => Instruction::Nop(number),
//             "acc" => Instruction::Accumulator(number),
//             "jmp" => Instruction::Jump(number),
//             _ => panic!("invalid raw value"),
//         }
//     }
// }

// #[derive(Debug)]
// enum SimulationResult {
//     Terminated(isize),
//     Loop(isize),
// }

// fn simulate(inputs: &Vec<Instruction>) -> SimulationResult {
//     let mut index = 0;
//     let mut accumulator: isize = 0;
//     let mut visited = HashSet::new();

//     while !visited.contains(&index) && index != inputs.len() {
//         visited.insert(index);

//         match inputs[index] {
//             Instruction::Nop(_) => {
//                 index += 1;
//             }
//             Instruction::Accumulator(number) => {
//                 accumulator += number;
//                 index += 1;
//             }
//             Instruction::Jump(offset) => {
//                 index = (index as isize + offset) as usize;
//             }
//         };
//     }

//     return if index == inputs.len() {
//         SimulationResult::Terminated(accumulator)
//     } else {
//         SimulationResult::Loop(accumulator)
//     };
// }

// #[test]
// fn day_08_example_part_one() {
//     let raw_input = r#"
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
// "#;

//     let test_input = raw_input
//         .lines()
//         .filter(|s| s != &"")
//         .map(|s| String::from(s))
//         .map(|s| Instruction::from(s))
//         .collect::<Vec<Instruction>>();

//     assert_eq!(part_one(test_input), 5);
// }

// #[test]
// fn day_08_example_part_two() {
//     let raw_input = r#"
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
// "#;

//     let test_input = raw_input
//         .lines()
//         .filter(|s| s != &"")
//         .map(|s| String::from(s))
//         .map(|s| Instruction::from(s))
//         .collect::<Vec<Instruction>>();

//     assert_eq!(part_two(test_input), 8);
// }
