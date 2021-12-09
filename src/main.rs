use std::env;

include!("input.rs");
include!("array_math.rs");
include!("day1/day1.rs");

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("{}", args[1]);

    match args[1].as_ref() {
        "day1" => run_day1("./inputs/day1.txt"),
        _ => println!("No valid config for {}", args[1]),
    }

}
