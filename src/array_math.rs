
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
pub fn count_true(in_arr: Vec<bool>) -> u64 {
    let mut count = 0;

    for val in in_arr {
        if val {
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
