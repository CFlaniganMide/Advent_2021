pub fn diff<T>(list: Vec<T>) -> Vec<T> where T: std::ops::Sub<Output = T>, T: Copy{

    let mut output = Vec::<T>::new();

    if list.len() == 0 {
        panic!("Can't diff a zero-length vector");
    }  else if list.len() == 1 {
        return output;
    }

    for i in 1..list.len() {
        output.push(list[i] - list[i - 1]);
    }

    return output;

}

// Applies a function f to Vec in_var and returns a Vec of the same shape with the type returned
// by f
pub fn apply<Tin, Tout>(in_arr: Vec<Tin>, f: impl Fn(Tin) -> Tout) -> Vec<Tout> {
    let mut out = Vec::<Tout>::new();

    for val in in_arr {
        out.push(f(val));
    }

    return out;

}

//
pub fn count_true(in_arr: &Vec<bool>) -> u64 {
    let mut count = 0;

    for val in in_arr {
        if *val {
            count += 1;
        }
    }

    return count;

}

pub fn convolve<T>(in_var: Vec<T>, kernel: Vec<T>, mode: &str) -> Vec<T>
    where T: std::ops::Mul<Output = T>, T: std::ops::AddAssign, T: Copy {

    let end: usize;
    let k_len = kernel.len();

    if k_len == 0 {
        panic!("kernel must have a length greater than 0")
    } else if k_len == 1 {
        return apply(in_var, |x| {x*kernel[0]});
    }

    match mode {
        "same" => (end = in_var.len() - k_len + 1),
        _ => panic!("mode {} is not valid", mode),
    }

    let mut conv_res: T;

    let mut out = Vec::<T>::new();

    for i in 0..end {
        conv_res = kernel[0]*in_var[i];
        for j in 1..kernel.len() {
            conv_res += kernel[j] * in_var[i + j];
        }
        out.push(conv_res);
    }

    return out;

}


pub fn binary_to_int(val: String) -> u64 {
    assert!(val.len() < 64);

    let mut output = 0;

    for (i, x) in val.chars().rev().enumerate() {
        match x {
            '0' => output += 0,
            '1' => output += 1 << i,
            _ => panic!("{} not a valid binary character", x)
        }
    }

    return output;

}

pub fn not_inplace(in_arr: &mut Vec<bool>) {
    for i in 0..in_arr.len() {
        in_arr[i] ^= true;
    }
}

pub fn args_where<T>(in_arr: &Vec<T>, index: &Vec<bool>) -> Vec<T>
    where T: Copy {

    if in_arr.len() != index.len() {
        panic!(
            "Both arrays must be of the same length, not {} and {}",
            in_arr.len(),
            index.len(),
        );
    }

    let mut out = Vec::<T>::new();

    for i in 0..index.len() {
        if index[i] {
            out.push(in_arr[i]);
        }
    }

    return out;

}

pub fn accumulate<T>(in_vec: &Vec<T>) -> T where T: std::ops::AddAssign + Clone {
    let mut out: T = in_vec[0].clone();

    for i in 1..in_vec.len() {
        out += in_vec[i].clone();
    }

    return out;

}
