use std::path;
use std::str::FromStr;

use crate::input;
use crate::submarine;
use crate::submarine::Instruction;

pub fn run_day2(filename: &str) {

    part1(input::input_as_iter::<submarine::Instruction>(path::Path::new(filename)));

    part2(input::input_as_iter::<i64>(path::Path::new(filename)));

}


fn part1(in_arr: Vec<Instruction>) -> u64{
    return 0;
}


fn part2(in_arr: Vec<i64>) -> u64{
    return 0;
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
        // let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let target = 150;

        assert_eq!(part1(test_input), target);
    }

    // #[test]
    // fn test_day2_part2() {
    //     let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    //     let target = 5;
    //
    //     assert_eq!(part2(test_input), target);
    // }
}