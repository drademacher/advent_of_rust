pub fn solve() -> String {
    return format!("First part: {}\nSecond part: {}", part_one(), part_two());
}

fn read_input_file() -> Vec<usize> {
    return include_str!("day_01.txt")
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
}

fn part_one() -> usize {
    let inputs = read_input_file();

    for n in &inputs {
        for m in &inputs {
            if n + m == 2020 {
                return n * m;
            }
        }
    }

    return 0;
}

fn part_two() -> usize {
    let inputs = read_input_file();

    for n in &inputs {
        for m in &inputs {
            for k in &inputs {
                if n + m + k == 2020 {
                    return n * m * k;
                }
            }
        }
    }

    return 0;
}
