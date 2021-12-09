use std::env;

mod day1;
mod day2;
mod input;
mod array_math;
mod submarine;

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("{}", args[1]);

    match args[1].as_ref() {
        "day1" => day1::run_day1("./inputs/day1.txt"),
        "day2" => day2::run_day2("./inputs/day2.txt"),
        _ => println!("No valid config for {}", args[1]),
    }

}
