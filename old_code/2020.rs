
mod 2020:day_01;
mod 2020:day_02;
// mod day_03;
// mod day_04;
// mod day_05;
// mod day_06;
// mod day_07;
// mod day_08;
// mod day_09;
// mod day_10;
// mod day_12;
// mod day_15;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().expect("Invalid day number");

    match day {
        1 => println!("{}", day_01::solve()),
        2 => println!("{}", day_02::solve()),
        // 3 => println!("{}", day_03::solve()),
        // 4 => println!("{}", day_04::solve()),
        // 5 => println!("{}", day_05::solve()),
        // 6 => println!("{}", day_06::solve()),
        // 7 => println!("{}", day_07::solve()),
        // 8 => println!("{}", day_08::solve()),
        // 9 => println!("{}", day_09::solve()),
        // 10 => println!("{}", day_10::solve()),
        // 12 => println!("{}", day_12::solve()),
        // 15 => println!("{}", day_15::solve()),
        _ => println!("Invalid input"),
    };
}