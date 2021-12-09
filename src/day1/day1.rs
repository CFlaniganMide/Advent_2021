use std::path;

fn run_day1(filename: &str) {

    day1_part1(filename);

    day1_part2(filename);

}

fn day1_part1(filename: &str) {

    let in_vals = input::input_as_iter::<i64>(path::Path::new(filename));

    let pos_diff = array_math::apply(array_math::diff::<i64>(in_vals), |x| x > 0);

    let n_pos = array_math::count_true(pos_diff);

    println!("Part 1: {} diffs greater than 0", n_pos);

}

fn day1_part2(filename: &str) {

    let in_vals = input::input_as_iter::<i64>(path::Path::new(filename));

    let convolved = array_math::convolve(in_vals, vec![1, 1, 1], "same");

    let pos_diff = array_math::apply(array_math::diff::<i64>(convolved), |x| x > 0);

    let n_pos = array_math::count_true(pos_diff);

    println!("Part 2: {} diffs greater than 0", n_pos);

}