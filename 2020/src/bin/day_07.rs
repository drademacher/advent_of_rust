// use regex::Regex;
// use std::collections::HashMap;
// use std::collections::HashSet;

// pub fn solve() -> String {
//     return format!(
//         "First part: {}\nSecond part: {}",
//         part_one(read_input_file()),
//         part_two(read_input_file())
//     );
// }

// fn read_input_file<'a>() -> Vec<&'a str> {
//     let inputs: Vec<&str> = include_str!("day_07.txt").lines().collect();
//     return inputs;
// }

// struct NumberedBag<'a> {
//     number: usize,
//     name: &'a str,
// }

// fn parse_input(inputs: Vec<&str>) -> HashMap<&str, Vec<NumberedBag>> {
//     return inputs.iter().map(|s| parse_line(&s)).collect();
// }

// fn parse_line(input: &str) -> (&str, Vec<NumberedBag>) {
//     let line = input.split(" contain ").collect::<Vec<_>>().clone();

//     let key = line[0].clone();

//     let regex = Regex::new(r"(\d+) (.+)").unwrap();

//     let values: Vec<NumberedBag> = if line[1].contains(", ") {
//         line[1]
//             .split(", ")
//             .map(|s| regex.captures_iter(s).next().unwrap())
//             .map(|cap| NumberedBag {
//                 number: cap[1].parse::<usize>().unwrap(),
//                 name: cap.get(2).unwrap().as_str().trim(),
//             })
//             .collect()
//     } else {
//         let cap = regex.captures_iter(line[1]).next().unwrap();
//         vec![NumberedBag {
//             number: cap[1].parse::<usize>().unwrap(),
//             name: cap.get(2).unwrap().as_str().trim(),
//         }]
//     };

//     return (key, values);
// }

// fn part_one(inputs: Vec<&str>) -> usize {
//     let edges = parse_input(inputs);

//     let mut visited = HashSet::new();
//     let mut frontier = Vec::new();
//     frontier.push("shiny gold bag");

//     while !frontier.is_empty() {
//         let current = frontier.pop().unwrap();
//         visited.insert(current);

//         for edge in &edges {
//             if edge.1.iter().filter(|x| x.name == current).count() > 0
//                 && !frontier.contains(&edge.0)
//             {
//                 frontier.push(edge.0);
//             }
//         }
//     }

//     return visited.len() - 1;
// }

// fn part_two(inputs: Vec<&str>) -> usize {
//     let edges = parse_input(inputs);

//     let mut result = 0;
//     let mut frontier = Vec::new();
//     frontier.push((1, "shiny gold bag"));

//     while !frontier.is_empty() {
//         let current = frontier.pop().unwrap();
//         let new_bags = edges.get(&current.1);

//         if new_bags.is_none() {
//             continue;
//         }

//         for new_bag in new_bags.unwrap() {
//             let multiplier = current.0 * new_bag.number;
//             result += multiplier;
//             frontier.push((multiplier, new_bag.name));
//         }
//     }

//     return result;
// }

// #[test]
// fn example_for_part_one() {
//     let test_inputs = vec![
//         "light red bag contain 1 bright white bag, 2 muted yellow bag",
//         "dark orange bag contain 3 bright white bag, 4 muted yellow bag",
//         "bright white bag contain 1 shiny gold bag",
//         "muted yellow bag contain 2 shiny gold bag, 9 faded blue bag",
//         "shiny gold bag contain 1 dark olive bag, 2 vibrant plum bag",
//         "dark olive bag contain 3 faded blue bag, 4 dotted black bag",
//         "vibrant plum bag contain 5 faded blue bag, 6 dotted black bag",
//     ];

//     assert_eq!(part_one(test_inputs), 4);
// }

// #[test]
// fn example_for_part_two() {
//     let test_inputs = vec![
//         "light red bag contain 1 bright white bag, 2 muted yellow bag",
//         "dark orange bag contain 3 bright white bag, 4 muted yellow bag",
//         "bright white bag contain 1 shiny gold bag",
//         "muted yellow bag contain 2 shiny gold bag, 9 faded blue bag",
//         "shiny gold bag contain 1 dark olive bag, 2 vibrant plum bag",
//         "dark olive bag contain 3 faded blue bag, 4 dotted black bag",
//         "vibrant plum bag contain 5 faded blue bag, 6 dotted black bag",
//     ];

//     assert_eq!(part_two(test_inputs), 32);
// }
