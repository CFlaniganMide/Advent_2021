use std::path;
use std::str::FromStr;

use crate::input;
use crate::submarine;
use crate::submarine::*;

pub fn run_day2(filename: &str) {

    println!(
        "part 1: {}",
        part1(input::input_as_iter::<submarine::Instruction>(path::Path::new(filename))),
    );

    println!(
        "part 2: {}",
        part2(input::input_as_iter::<submarine::Instruction>(path::Path::new(filename))),
    );

}


fn part1(in_arr: Vec<Instruction>) -> i64{
    let mut sub = Submarine::new();

    for instr in in_arr {
        sub.do_instruction_part1(instr);
    }

    return sub.get_horizontal()*sub.get_depth();
}


fn part2(in_arr: Vec<submarine::Instruction>) -> i64{
    let mut sub = Submarine::new();

    for instr in in_arr {
        sub.do_instruction_part2(instr);
    }

    return sub.get_horizontal()*sub.get_depth();
}


#[cfg(test)]
mod tests {

    use crate::day2::*;

    #[test]
    fn test_day2_part1() {

        let test_str = String::from("forward 5\r\ndown 5\r\nforward 8\r\nup 3\r\ndown 8\r\nforward 2");
        let mut test_input = Vec::<Instruction>::new();
        for line in test_str.lines() {
            test_input.push(Instruction::from_str(line).unwrap());
        }
        let target = 150;

        assert_eq!(part1(test_input), target);
    }

    #[test]
    fn test_day2_part2() {

        let test_str = String::from("forward 5\r\ndown 5\r\nforward 8\r\nup 3\r\ndown 8\r\nforward 2");
        let mut test_input = Vec::<Instruction>::new();
        for line in test_str.lines() {
            test_input.push(Instruction::from_str(line).unwrap());
        }
        let target = 900;

        assert_eq!(part2(test_input), target);
    }
}