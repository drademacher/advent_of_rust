pub fn result() -> String {
    return format!("First part: {}\nSecond part: {}", part_one(), part_two());
}

fn part_one() -> usize {
    let raw_input: &'static str = include_str!("day_01.txt");
    let input: Vec<usize> = raw_input.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    for n in &input {
        for m in &input {
            if n + m == 2020 {
                return n * m;
            }
        }
    }

    return 0;
}

fn part_two() -> usize {
    let raw_input: &'static str = include_str!("day_01.txt");
    let input: Vec<usize> = raw_input.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    for n in &input {
        for m in &input {
            for k in &input {
                if n + m + k == 2020 {
                    return n * m * k;
                }
            }
        }
    }

    return 0;
}