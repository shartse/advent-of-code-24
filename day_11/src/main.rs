use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let stones: Vec<u64> = values
        .first()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    solve(&stones, 25);
    solve(&stones, 75);
}

fn solve(stones: &Vec<u64>, iters: u32) {
    let ans: u64 = stones
        .iter()
        .map(|&s| stones_produced(s, iters, &mut HashMap::new()))
        .sum();
    println!("Total value is: {ans}");
}

fn stones_produced(val: u64, blinks_left: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }
    if let Some(stones) = memo.get(&(val, blinks_left)) {
        return *stones;
    }
    let ans = if val == 0 {
        stones_produced(1, blinks_left - 1, memo)
    } else {
        let str_rep = val.to_string();
        let len = str_rep.len();
        if len % 2 == 0 {
            let a = str_rep[..len / 2].parse::<u64>().unwrap();
            let b = str_rep[len / 2..].parse::<u64>().unwrap();
            stones_produced(a, blinks_left - 1, memo) + stones_produced(b, blinks_left - 1, memo)
        } else {
            stones_produced(val * 2024, blinks_left - 1, memo)
        }
    };
    memo.insert((val, blinks_left), ans);
    return ans;
}
