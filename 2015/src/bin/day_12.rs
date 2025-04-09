#![allow(dead_code, unused_imports, unused_variables)]

use serde_json::Value;

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn read_input_file() -> &'static str {
    include_str!("../../resources/day_12.txt")
}

fn part_one(input: &str) -> i32 {
    let json: Value = serde_json::from_str(input).expect("Invalid JSON");

    fn sum_numbers(value: &Value) -> i32 {
        match value {
            Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
            Value::Array(arr) => arr.iter().map(sum_numbers).sum(),
            Value::Object(obj) => obj.values().map(sum_numbers).sum(),
            _ => 0,
        }
    }

    sum_numbers(&json)
}

fn part_two(input: &str) -> i32 {
    let json: Value = serde_json::from_str(input).expect("Invalid JSON");

    fn sum_numbers(value: &Value) -> i32 {
        match value {
            Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
            Value::Array(arr) => arr.iter().map(sum_numbers).sum(),
            Value::Object(obj) => {
                let contains_value_red = obj
                    .values()
                    .any(|val| val.is_string() && val.as_str().unwrap() == "red");

                if !contains_value_red {
                    obj.values().map(sum_numbers).sum()
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    sum_numbers(&json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(r#"[1,2,3]"#), 6);
        assert_eq!(part_one(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(part_one(r#"[[[3]]]"#), 3);
        assert_eq!(part_one(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(part_one(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(part_one(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(part_one(r#"[]"#), 0);
        assert_eq!(part_one(r#"{}"#), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(r#"[1,2,3]"#), 6);
        assert_eq!(part_two(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(part_two(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(part_two(r#"[1,"red",5]"#), 6);
    }
}
