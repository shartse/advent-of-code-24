use std::{
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

fn main() {
    let file = File::open("test.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let lines: Vec<(i64, Vec<i64>)> = values
        .into_iter()
        .map(|v| {
            let (n, expr) = v.split_once(':').unwrap();
            let n: i64 = n.parse().unwrap();
            let expr: Vec<i64> = expr
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (n, expr)
        })
        .collect();
    part_1_and_2(lines.clone());
}

fn valid(target: i64, nums: &Vec<i64>, ops: &Vec<Op>) -> bool {
    let mut running_total = *nums.first().unwrap();
    for (i, op) in ops.iter().enumerate() {
        let cur_num = nums[i + 1];
        match op {
            Op::Add => {
                running_total += cur_num;
            }
            Op::Mult => {
                running_total *= cur_num;
            }
            Op::Concat => {
                running_total = format!("{running_total}{cur_num}").parse::<i64>().unwrap()
            }
        }
    }
    running_total == target
}

fn possible_ops(n: usize) -> Vec<Vec<Op>> {
    let mut ops = vec![vec![Op::Add], vec![Op::Mult], vec![Op::Concat]];
    for i in 1..n {
        let mut new_ops = Vec::new();
        for each in ops.iter() {
            let mut a = each.clone();
            let mut b = each.clone();
            let mut c = each.clone();
            a.push(Op::Add);
            b.push(Op::Mult);
            c.push(Op::Concat);
            new_ops.push(a);
            new_ops.push(b);
            new_ops.push(c);
        }
        ops = new_ops;
    }
    return ops;
}

fn part_1_and_2(lines: Vec<(i64, Vec<i64>)>) {
    let mut ans = 0;
    for (target, nums) in lines {
        for ops in possible_ops(nums.len() - 1).iter().unique() {
            if valid(target, &nums, &ops) {
                println!("{:?} with ops {:?} comes to: {:?}", nums, ops, target);
                ans += target;
                break;
            }
        }
    }
    println!("Total value is: {ans}");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Op {
    Add,
    Mult,
    Concat, // for part 1, remove all references to Concat
}
