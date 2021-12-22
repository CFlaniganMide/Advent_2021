use std::path;

use crate::geometry;
use crate::input;


pub fn run_day5(filename: &str) {

    println!(
        "part 1: {}",
        part1(input::input_as_iter_lines::<geometry::Line2d>(path::Path::new(filename))),
    );

    println!(
        "part 2: {}",
        part2(input::input_as_iter_lines::<geometry::Line2d>(path::Path::new(filename))),
    );

}

fn part1(in_vals: Vec<geometry::Line2d>) -> u64{

    let mut max_x = 0;
    let mut max_y = 0;

    for line in &in_vals {
        if line.max_x() > max_x {
            max_x = line.max_x().clone();
        }
        if line.max_y() > max_y {
            max_y = line.max_y().clone();
        }
    }

    let mut field = geometry::Plane2d::new(max_x + 1, max_y + 1);
    for line in in_vals {
        if line.vertical() || line.horizontal() {
            field.mark(line);
        }
    }

    let mut count = 0;

    for v in field {
        if v > 1 {
            count += 1;
        }
    }

    return count;

}

fn part2(in_vals: Vec<geometry::Line2d>) -> u64{

    let mut max_x = 0;
    let mut max_y = 0;

    for line in &in_vals {
        if line.max_x() > max_x {
            max_x = line.max_x().clone();
        }
        if line.max_y() > max_y {
            max_y = line.max_y().clone();
        }
    }

    let mut field = geometry::Plane2d::new(max_x + 1, max_y + 1);
    for line in in_vals {
        field.mark(line);
    }

    let mut count = 0;

    for v in field {
        if v > 1 {
            count += 1;
        }
    }

    return count;

}


#[cfg(test)]
mod tests {

    use crate::day5::*;

    #[test]
    fn test_day5_part1() {
        let test_file = "./inputs/test/day5.txt";
        let test_input = input::input_as_iter_lines::<geometry::Line2d>(
            path::Path::new(test_file),
        );
        let target = 5;

        assert_eq!(part1(test_input), target);
    }

    #[test]
    fn test_day5_part2() {
        let test_file = "./inputs/test/day5.txt";
        let test_input = input::input_as_iter_lines::<geometry::Line2d>(
            path::Path::new(test_file),
        );
        let target = 12;

        assert_eq!(part2(test_input), target);
    }
}