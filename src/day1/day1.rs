use std::path;

fn run_day1(filename: &str) {

    day1_part1(input::input_as_iter::<i64>(path::Path::new(filename)));

    day1_part2(input::input_as_iter::<i64>(path::Path::new(filename)));

}

fn day1_part1(in_vals: Vec<i64>) -> u64{

    let pos_diff = array_math::apply(
        array_math::diff::<i64>(in_vals),
        |x| x > 0,
    );

    let n_pos = array_math::count_true(pos_diff);

    println!("Part 1: {} diffs greater than 0", n_pos);

    return n_pos;

}

fn day1_part2(in_vals: Vec<i64>) -> u64{

    let convolved = array_math::convolve(
        in_vals,
        vec![1, 1, 1],
        "same",
    );

    let pos_diff = array_math::apply(
        array_math::diff::<i64>(convolved),
        |x| x > 0,
    );

    let n_pos = array_math::count_true(pos_diff);

    println!("Part 2: {} diffs greater than 0", n_pos);

    return n_pos;

}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_day1_part1() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let target = 7;

        assert_eq!(day1_part1(test_input), target);
    }

    #[test]
    fn test_day1_part2() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let target = 5;

        assert_eq!(day1_part2(test_input), target);
    }
}