use std::path;

use crate::input;
use crate::array_math;
use crate::array_math::accumulate;
use crate::lanternfish;


pub fn run_day6(filename: &str) {

    println!(
        "part 1: {}",
        part1(input::input_as_iter::<u64>(path::Path::new(filename), ",")),
    );

    println!(
        "part 2: {}",
        part2(input::input_as_iter::<u64>(path::Path::new(filename), ",")),
    );

}

fn part1(in_vals: Vec<u64>) -> u64{

    let fish = array_math::apply(in_vals, |x| lanternfish::evolve(x, 80));

    return accumulate(&fish);

}

fn part2(in_vals: Vec<u64>) -> u64{

    let fish = array_math::apply(in_vals, |x| lanternfish::evolve(x, 256));

    return accumulate(&fish);

}


#[cfg(test)]
mod tests {

    use crate::day6::*;

    #[test]
    fn test_day6_part1() {
        let test_input = vec![3,4,3,1,2];
        let target = 5934;

        assert_eq!(part1(test_input), target);
    }

    #[test]
    fn test_day6_part2() {
        let test_input = vec![3,4,3,1,2];
        let target = 5;

        assert_eq!(part2(test_input), target);
    }
}