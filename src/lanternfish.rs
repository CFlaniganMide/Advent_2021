pub fn evolve(term: u64, generations: u64) -> u64{
    return evolve_memo(term as usize, generations as usize, &mut Vec::<u64>::new());
}


pub fn evolve_memo(term: usize, generations: usize, memo: &mut Vec<u64>) -> u64 {
    if memo.len() < ((generations + 1)*9) as usize {
        for _ in memo.len()..((generations + 1)*9) {
            memo.push(0);
        }
    }

    if memo[term + generations*9] == 0 {
        if generations <= term {
            memo[term + generations*9] = 1;
        } else {
            memo[term + generations*9] = evolve_memo(6, generations - term - 1, memo) + evolve_memo(8, generations - term - 1, memo);
        }
    }

    return memo[term + generations*9];
}