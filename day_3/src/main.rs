use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let line: String = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("");
    part_1(line.clone());
    part_2(line);
}

fn part_1(line: String) {
    let re = Regex::new(r"(mul\(\d{1,3}\,\d{1,3}\))").unwrap();
    let ans: u32 = re
        .captures_iter(&line)
        .map(|c| c.extract())
        .map(|(_, [expr])| mult(expr))
        .sum();
    println!("Total value is: {ans}");
}

fn part_2(line: String) {
    let mut mult_on = true;
    let mut exprs = Vec::new();
    let re = Regex::new(r"(mul\(\d{1,3}\,\d{1,3}\)|do\(\)|don\'t\(\))").unwrap();
    for (_, [m]) in re.captures_iter(&line).map(|c| c.extract()) {
        if m == "do()" {
            mult_on = true;
        } else if m == "don't()" {
            mult_on = false
        } else {
            if mult_on {
                exprs.push(m);
            }
        }
    }
    let ans: u32 = exprs.into_iter().map(|expr| mult(expr)).sum();
    println!("Total value is: {ans}");
}

fn mult(expr: &str) -> u32 {
    let nums = expr.trim_start_matches("mul(").trim_end_matches(")");
    let (a, b) = nums.split_once(',').unwrap();
    a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
}