use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod array_math;
mod bingo;
mod diagnostics;
mod geometry;
mod input;
mod submarine;

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("{}", args[1]);

    match args[1].as_ref() {
        "day1" => day1::run_day1("./inputs/day1.txt"),
        "day2" => day2::run_day2("./inputs/day2.txt"),
        "day3" => day3::run_day3("./inputs/day3.txt"),
        "day4" => day4::run_day4("./inputs/day4.txt"),
        "day5" => day5::run_day5("./inputs/day5.txt"),
        _ => println!("No valid config for {}", args[1]),
    }

}
