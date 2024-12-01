use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    part_1(values.clone());
    part_2(values);
}

fn part_1(vals: Vec<String>) {
    let values: Vec<(i32, i32)> = vals
        .iter()
        .map(|line| {
            let (l, r) = line.split_once(char::is_whitespace).unwrap();
            (
                l.trim().parse::<i32>().unwrap(),
                r.trim().parse::<i32>().unwrap(),
            )
        })
        .collect();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = values.into_iter().unzip();
    left.sort();
    right.sort();
    let ans = left
        .iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r));
    println!("Total value is: {ans}");
}

fn part_2(vals: Vec<String>) {
    let values: Vec<(i32, i32)> = vals
        .iter()
        .map(|line| {
            let (a, b) = line.split_once(char::is_whitespace).unwrap();
            (
                a.trim().parse::<i32>().unwrap(),
                b.trim().parse::<i32>().unwrap(),
            )
        })
        .collect();
    let (left, right): (Vec<i32>, Vec<i32>) = values.into_iter().unzip();
    let mut ans = 0;
    for l in left {
        let count = right.iter().filter(|&x| *x == l).count();
        ans += l * count as i32
    }
    println!("Total value is: {ans}");
}
