use std::{
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let machines_part_1: Vec<Machine> = values
        .chunks(4)
        .map(|c| Machine::new(c.to_vec(), false))
        .collect();
    solve(&machines_part_1);

    let machines_part_2: Vec<Machine> = values
        .chunks(4)
        .map(|c| Machine::new(c.to_vec(), true))
        .collect();
    solve(&machines_part_2);
}

fn solve(machines: &Vec<Machine>) {
    let mut ans = 0;
    for m in machines.iter() {
        if let Some((a, b)) = m.solve() {
            ans += (a * 3) + b;
        }
    }
    println!("Total value is: {ans}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos(i64, i64);
impl Pos {
    fn parse_button(line: &str) -> Pos {
        let re = Regex::new(r"Button \w: X\+(?<X>\d*), Y\+(?<Y>\d*)").unwrap();
        let caps = re.captures(line).unwrap();
        let x = caps["X"].parse::<i64>().unwrap();
        let y = caps["Y"].parse::<i64>().unwrap();
        return Pos(x, y);
    }

    fn parse_prize(line: &str, part_2: bool) -> Pos {
        let re = Regex::new(r"Prize: X\=(?<X>\d*), Y\=(?<Y>\d*)").unwrap();
        let caps = re.captures(line).unwrap();
        let x = caps["X"].parse::<i64>().unwrap();
        let y = caps["Y"].parse::<i64>().unwrap();
        if part_2 {
            return Pos(x + 10000000000000, y + 10000000000000);
        }
        return Pos(x, y);
    }
}
#[derive(Debug, Clone)]
struct Machine {
    prize: Pos,
    button_a: Pos,
    button_b: Pos,
}

impl Machine {
    fn new(lines: Vec<String>, part_2: bool) -> Self {
        Machine {
            button_a: Pos::parse_button(&lines[0]),
            button_b: Pos::parse_button(&lines[1]),
            prize: Pos::parse_prize(&lines[2], part_2),
        }
    }

    fn solve(&self) -> Option<(i64, i64)> {
        let (ax, ay) = (self.button_a.0, self.button_a.1);
        let (bx, by) = (self.button_b.0, self.button_b.1);
        let (px, py) = (self.prize.0, self.prize.1);

        let a = ((bx * py) - (by * px)) / ((ay * bx) - (ax * by));

        let b = (px - (a * ax)) / bx;

        let px_aprox = a * ax + b * bx;
        let py_aprox = a * ay + b * by;

        if px_aprox == self.prize.0 && py_aprox == self.prize.1 {
            return Some((a, b));
        } else {
            return None;
        }
    }
}
