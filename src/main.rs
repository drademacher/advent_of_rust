mod day_01;
mod day_02;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().expect("Invalid day number");

    match day {
        1 => println!("{}", day_01::solve()),
        2 => println!("{}", day_02::solve()),
        _ => println!("Invalid input"),
    };
}
