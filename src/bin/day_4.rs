use md5::{Digest, Md5};

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file()),
        part_two(read_input_file())
    );
}

fn read_input_file() -> &'static str {
    return include_str!("../../resources/2015/day_4.txt")
        .lines()
        .collect::<Vec<&str>>()[0];
}

fn part_one(secret_key: &'static str) -> i32 {
    let mut number = 1;

    loop {
        let hash_as_bytes = Md5::new()
            .chain_update(format!("{}{}", secret_key, number))
            .finalize();
        let first_five_hex_chars_are_zero =
            hash_as_bytes[0] == 0 && hash_as_bytes[1] == 0 && hash_as_bytes[2] / 16 == 0;

        if first_five_hex_chars_are_zero {
            break;
        }

        number += 1;
    }

    return number;
}

fn part_two(secret_key: &'static str) -> u32 {
    let mut number = 1;

    loop {
        let hash_as_bytes = Md5::new()
            .chain_update(format!("{}{}", secret_key, number))
            .finalize();
        let first_six_hex_chars_are_zero =
            hash_as_bytes[0] == 0 && hash_as_bytes[1] == 0 && hash_as_bytes[2] == 0;

        if first_six_hex_chars_are_zero {
            break;
        }

        number += 1;
    }

    return number;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("abcdef"), 609043);
        assert_eq!(part_one("pqrstuv"), 1048970);
    }
}
