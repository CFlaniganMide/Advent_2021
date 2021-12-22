use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct Card {
    pub values: [[u64; 5]; 5],
    pub called: [[bool; 5]; 5],
}

impl Card {

    pub fn new() -> Card{
        return Card {
            values: [[0; 5]; 5],
            called: [[false; 5]; 5],
        }
    }

    pub fn call(&mut self, val: u64) {
        for x in 0..5 {
            for y in 0..5 {
                if self.values[x][y] == val {
                    self.called[x][y] = true;
                }
            }
        }
    }

    pub fn reset(&mut self) {

        for x in 0..5 {
            for y in 0..5 {
                self.called[x][y] = false;
            }
        }
    }

    pub fn winner(&self) -> bool {

        for i in 0..5 {

            if self.called[i].iter().all(|x| *x) {
                return true;
            }

            if self.called.iter().all(|x| x[i]) {
                return true;
            }

        }

        return false;

    }

}

impl Display for Card {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {

        let mut output = String::from("+--+--+--+--+--+");

        for y in 0..5 {
            output += "\n";
            for x in 0..5 {
                output += &format!("|{:2}", self.values[x][y]);
            }

            output += "|\n+--+--+--+--+--+";
        }

        return formatter.write_str(output.as_str());

    }

}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut out = Card::new();

        let rows = s.lines().collect::<Vec<&str>>();
        if rows.len() != 5 {
            return Err(());
        }

        for (y, row) in rows.into_iter().enumerate() {
            let vals = row.split_whitespace().collect::<Vec<&str>>();
            if vals.len() != 5 {
                return Err(());
            }

            for (x, v) in vals.into_iter().enumerate() {
                out.values[x][y] = v.parse().unwrap();
            }
        }

        return Ok(out);

    }
}


pub fn parse(input: String) -> (Vec<u64>, Vec<Card>) {
    let mut filtered_input = String::new();
    for substring in input.split('\r') {
        filtered_input.push_str(substring);
    }
    let double_newline = "\n\n";
    let var = filtered_input.split(double_newline);
    let mut calls = Vec::<u64>::new();
    let mut cards = Vec::<Card>::new();

    for (i, v) in var.enumerate() {
        if i == 0 {
            for c in v.split(',') {
                calls.push(c.parse().unwrap());
            }
        } else {
            cards.push(Card::from_str(v).unwrap());
        }
    }

    return (calls, cards);

}