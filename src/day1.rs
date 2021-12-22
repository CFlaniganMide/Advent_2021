use std::path;

use crate::input;
use crate::array_math;

pub fn run_day1(filename: &str) {

    println!(
        "part 1: {}",
        part1(input::input_as_iter_lines::<i64>(path::Path::new(filename))),
    );

    println!(
        "part 2: {}",
        part2(input::input_as_iter_lines::<i64>(path::Path::new(filename))),
    );

}

fn part1(in_vals: Vec<i64>) -> u64{

    let pos_diff = array_math::apply(
        array_math::diff::<i64>(in_vals),
        |x| x > 0,
    );

    let n_pos = array_math::count_true(&pos_diff);

    return n_pos;

}

fn part2(in_vals: Vec<i64>) -> u64{

    let convolved = array_math::convolve(
        in_vals,
        vec![1, 1, 1],
        "same",
    );

    let pos_diff = array_math::apply(
        array_math::diff::<i64>(convolved),
        |x| x > 0,
    );

    let n_pos = array_math::count_true(&pos_diff);

    return n_pos;

}


#[cfg(test)]
mod tests {

    use crate::day1::*;

    #[test]
    fn test_day1_part1() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let target = 7;

        assert_eq!(part1(test_input), target);
    }

    #[test]
    fn test_day1_part2() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let target = 5;

        assert_eq!(part2(test_input), target);
    }
}