use std::path::Path;

use crate::input;
use crate::bingo;

pub fn run_day4(filename: &str) {

    let (calls, cards) = bingo::parse(input::input_as_string(Path::new(filename)));

    println!(
        "part 1: {}",
        part1(calls, cards));

    let (calls, cards) = bingo::parse(input::input_as_string(Path::new(filename)));

    println!(
        "part 2: {}",
        part2(calls, cards));

}


fn part1(calls: Vec<u64>, mut cards: Vec<bingo::Card>) -> i64 {

    for call in calls {
        for i in 0..cards.len() {
            cards[i].call(call);

            if cards[i].winner() {
                let mut a = 0;
                let b = call;
                for x in 0..5 {
                    for y in 0..5 {
                        if !cards[i].called[x][y] {
                            a += cards[i].values[x][y];
                        }
                    }
                }

                return (a * b) as i64;
            }
        }
    }

    panic!("Didn't find a winner for day 4 part 1");
}


fn part2(calls: Vec<u64>, mut cards: Vec<bingo::Card>) -> i64{

    for call in calls {

        let mut losers = Vec::<bingo::Card>::new();

        for i in 0..cards.len() {
            cards[i].call(call);

            if !cards[i].winner() {
                losers.push(cards[i]);
            }
        }

        if losers.len() == 0 {
            let mut a = 0;
            let b = call;
            for x in 0..5 {
                for y in 0..5 {
                    if !cards[0].called[x][y] {
                        a += cards[0].values[x][y];
                    }
                }
            }

            return (a * b) as i64;
        }

        cards = losers;

    }

    panic!("Didn't find a winner for day 4 part 2");
}


#[cfg(test)]
mod tests {

    use crate::day4::*;

    #[test]
    fn test_day4_part1() {

        let input = String::from(
"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");

        let (test_calls, test_cards) = bingo::parse(input);

        let target = 4512;

        assert_eq!(part1(test_calls, test_cards), target);
    }

    #[test]
    fn test_day4_part2() {

        let input = String::from(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");

        let (test_calls, test_cards) = bingo::parse(input);

        let target = 1924;

        assert_eq!(part2(test_calls, test_cards), target);
    }
}