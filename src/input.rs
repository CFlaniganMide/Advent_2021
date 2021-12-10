pub fn input_as_string(filename: &std::path::Path) -> String {

    let file = std::fs::read_to_string(filename);

    if file.is_err() {
        panic!("something went wrong");
    } else {
        return file.unwrap();
    }

}

pub fn input_as_iter<T: std::str::FromStr>(filename: &std::path::Path) -> Vec<T>
    where T: std::str::FromStr,
          <T as std::str::FromStr>::Err: std::fmt::Debug {

    let mut output = Vec::<T>::new();

    for val in std::fs::read_to_string(filename).unwrap().lines() {

        output.push(val.parse().unwrap());

    }

    return output;

}