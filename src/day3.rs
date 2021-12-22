use std::path::Path;

use crate::input;
use crate::array_math;
use crate::diagnostics::DiagnosticArray;

pub fn run_day3(filename: &str) {

    println!(
        "part 1: {}",
        part1(
            array_math::apply(
                input::input_as_iter_lines::<String>(Path::new(filename)),
                array_math::binary_to_int,
            ),
            input::input_width(Path::new(filename)) as usize,
        ),
    );

    println!(
        "part 2: {}",
        part2(
            array_math::apply(
                input::input_as_iter_lines::<String>(Path::new(filename)),
                array_math::binary_to_int,
            ),
            input::input_width(Path::new(filename)) as usize,
        ),
    );

}


fn part1(in_arr: Vec<u64>, width: usize) -> i64{

    let diagnostic = DiagnosticArray::new(in_arr, width);

    // return (gamma * epsilon) as i64;
    return diagnostic.gamma()*diagnostic.epsilon();
}



fn part2(in_arr: Vec<u64>, width: usize) -> i64{

    let diagnostic = DiagnosticArray::new(in_arr, width);

    diagnostic.oxygen_gen_rating();

    // return (gamma * epsilon) as i64;
    return diagnostic.oxygen_gen_rating()*diagnostic.c02_scrub_rating();
}


#[cfg(test)]
mod tests {

    use crate::day3::*;

    #[test]
    fn test_day3_part1() {

        let test_input = vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010,
        ];

        let target = 198;

        assert_eq!(part1(test_input, 5), target);
    }

    #[test]
    fn test_day3_part2() {

        let test_input = vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010,
        ];

        let target = 230;

        assert_eq!(part2(test_input, 5), target);
    }
}