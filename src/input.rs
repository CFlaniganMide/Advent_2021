use std::path::Path;

pub fn input_as_string(filename: &Path) -> String {

    let file = std::fs::read_to_string(filename);

    if file.is_err() {
        panic!("something went wrong");
    } else {
        return file.unwrap();
    }

}

pub fn input_as_iter_lines<T: std::str::FromStr>(filename: &Path) -> Vec<T>
    where T: std::str::FromStr,
          <T as std::str::FromStr>::Err: std::fmt::Debug {

    let mut output = Vec::<T>::new();

    for val in std::fs::read_to_string(filename).unwrap().lines() {

        output.push(val.parse().unwrap());

    }

    return output;

}

pub fn input_as_iter<T: std::str::FromStr>(filename: &Path, sep: &str) -> Vec<T>
    where T: std::str::FromStr,
          <T as std::str::FromStr>::Err: std::fmt::Debug {

    let mut output = Vec::<T>::new();

    for val in std::fs::read_to_string(filename).unwrap().split(sep) {

        output.push(val.parse().unwrap());

    }

    return output;

}

pub fn input_width(filename: &Path) -> u64 {

    let mut output = 0;

    for val in std::fs::read_to_string(filename).unwrap().lines() {
        output = val.len();
    }

    return output as u64;
}