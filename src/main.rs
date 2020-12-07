mod day_01;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().expect("Invalid day number");

    match day {
        1 => println!("{}", day_01::result()),
        _ => println!("Invalid input")
    };
}
