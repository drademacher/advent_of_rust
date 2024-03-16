#![allow(dead_code, unused_imports, unused_variables)]

use std::str::FromStr;

use regex::Regex;

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn part_one(actions: Vec<Action>) -> u64 {
    let mut map = [[false; 1000]; 1000];

    for action in actions {
        for i in action.1..=action.3 {
            for j in action.2..=action.4 {
                match action.0 {
                    ActionType::TurnOn => map[i][j] = true,
                    ActionType::TurnOff => map[i][j] = false,
                    ActionType::Toggle => map[i][j] = !map[i][j],
                }
            }
        }
    }

    return map
        .map(|array| array.map(|b| b as u64).iter().sum())
        .iter()
        .sum();
}

fn part_two(actions: Vec<Action>) -> u64 {
    let mut map = [[0u64; 1000]; 1000];

    for action in actions {
        for i in action.1..=action.3 {
            for j in action.2..=action.4 {
                match action.0 {
                    ActionType::TurnOn => map[i][j] += 1,
                    ActionType::TurnOff => {
                        if map[i][j] > 0 {
                            map[i][j] -= 1
                        }
                    }
                    ActionType::Toggle => map[i][j] += 2,
                }
            }
        }
    }

    return map
        .iter()
        .map(|array| array.iter().map(|b| b).sum::<u64>())
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(vec![Action(ActionType::TurnOn, 0, 0, 999, 999)]),
            10000
        );
    }

    #[test]
    fn test_part_two() {
        // assert_eq!(part_two(""), );
    }
}

fn read_input_file() -> Vec<Action> {
    return include_str!("../../resources/2015/day_6.txt")
        .lines()
        .map(|s| {
            s.parse::<Action>()
                .expect(format!("cant parse '{}'", s).as_str())
        })
        .collect();
}

#[derive(PartialEq, Debug)]
struct Action(ActionType, usize, usize, usize, usize);

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex =
            Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        let captures = regex.captures(s).ok_or("Regex failed")?;

        let action_type = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<ActionType>()
            .expect("Failed");

        Ok(Action(
            action_type,
            group_as_u32(&captures, 2),
            group_as_u32(&captures, 3),
            group_as_u32(&captures, 4),
            group_as_u32(&captures, 5),
        ))
    }
}

fn group_as_u32(captures: &regex::Captures<'_>, group_number: usize) -> usize {
    captures
        .get(group_number)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap()
}

#[derive(PartialEq, Debug)]
enum ActionType {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for ActionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(ActionType::TurnOn),
            "turn off" => Ok(ActionType::TurnOff),
            "toggle" => Ok(ActionType::Toggle),
            _ => Err(format!("failed for {}", s)),
        }
    }
}
