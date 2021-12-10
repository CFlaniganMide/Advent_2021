use std::str::FromStr;

pub struct Submarine {
    horizontal: i64,
    depth: i64,
    aim: i64,
}


pub struct Instruction {
    direction: String,
    value: i64,
}

impl Submarine {

    pub fn do_instruction_part1(&mut self, instr: Instruction) {
        match instr.direction.as_str() {
            "forward" => self.horizontal += instr.value,
            "up" => self.depth -= instr.value,
            "down" => self.depth += instr.value,
            _ => panic!("direction {} is invalid", instr.direction),
        }

    }

    pub fn do_instruction_part2(&mut self, instr: Instruction) {
        match instr.direction.as_str() {
            "forward" => {
                self.horizontal += instr.value;
                self.depth += self.aim*instr.value;
            },
            "up" => self.aim -= instr.value,
            "down" => self.aim += instr.value,
            _ => panic!("direction {} is invalid", instr.direction),
        }

    }

    pub fn new() -> Submarine {
        return Submarine {horizontal: 0, depth: 0, aim: 0}
    }

    pub fn get_horizontal(&self) -> i64 {
        return self.horizontal;
    }

    pub fn get_depth(&self) -> i64 {
        return self.depth;
    }

    pub fn get_aim(&self) -> i64 {
        return self.aim;
    }

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
