use std::str::FromStr;

pub struct Submarine {
    horizontal: i64,
    depth: i64,
}


pub struct Instruction {
    direction: String,
    value: i64,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s.split_whitespace().nth(0).unwrap();
        let val = s.split_whitespace().nth(1).unwrap();
        return Result::Ok(
            Instruction {
                direction: String::from(dir),
                value: val.parse().unwrap(),
            }
        )
    }
}
