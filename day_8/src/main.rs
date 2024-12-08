use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let max = Point {
        row: values.len() as i64,
        col: values[0].len() as i64,
    };
    let mut map = AntennaMap {
        items: HashMap::new(),
    };
    for (row, line) in values.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                if !map.items.contains_key(&c) {
                    map.items.insert(c, Vec::new());
                }
                let points = map.items.get_mut(&c).unwrap();
                points.push(Point {
                    row: row as i64,
                    col: col as i64,
                });
            }
        }
    }

    part_1(map.clone(), &max);
    part_2(map, &max);
}

fn part_1(map: AntennaMap, max: &Point) {
    let mut antinodes = HashSet::new();
    for (antenna, points) in map.items.into_iter() {
        println!("Checking antenna: {:?}", antenna);
        for pair in points.into_iter().permutations(2) {
            let (a, b) = (pair[0].clone(), pair[1].clone());
            let line = Line { a, b };
            let (antinode_a, antinode_b) = line.two_antinodes();
            if antinode_a.inbounds(&max) {
                antinodes.insert(antinode_a);
            }
            if antinode_b.inbounds(&max) {
                antinodes.insert(antinode_b);
            }
        }
    }
    println!("Total value is: {:?}", antinodes.len());
}

fn part_2(map: AntennaMap, max: &Point) {
    let mut antinodes = HashSet::new();
    for (antenna, points) in map.items.into_iter() {
        println!("Checking antenna: {:?}", antenna);
        for pair in points.into_iter().permutations(2) {
            let (a, b) = (pair[0].clone(), pair[1].clone());
            let line = Line { a, b };
            for antinode in line.all_antinodes(&max) {
                antinodes.insert(antinode);
            }
        }
    }
    println!("Total value is: {:?}", antinodes.len());
}

#[derive(Debug, Clone)]
struct AntennaMap {
    items: HashMap<char, Vec<Point>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: i64,
    col: i64,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }

    fn sub(&self, other: &Point) -> Point {
        Point {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }

    fn inbounds(&self, max: &Point) -> bool {
        return self.row < max.row && self.col < max.col && self.row >= 0 && self.col >= 0;
    }
}

#[derive(Debug, Clone)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn diff(&self) -> Point {
        Point {
            row: self.b.row - self.a.row,
            col: self.b.col - self.a.col,
        }
    }

    fn gcd_diff(&self) -> Point {
        let diff = self.diff();

        let gcd = gcd::binary_u64(diff.row.abs() as u64, diff.col.abs() as u64) as i64;
        let new_row = diff.row / gcd;
        let new_col = diff.col / gcd;

        Point {
            row: new_row,
            col: new_col,
        }
    }

    fn two_antinodes(&self) -> (Point, Point) {
        let diff = self.diff();
        (self.a.sub(&diff), self.b.add(&diff))
    }

    fn all_antinodes(&self, max: &Point) -> HashSet<Point> {
        let gcd_diff = self.gcd_diff();
        let mut antinodes = HashSet::new();
        let mut start = self.a.clone();
        loop {
            let antinode = start.sub(&gcd_diff);
            if antinode.inbounds(max) {
                antinodes.insert(antinode.clone());
            } else {
                break;
            }
            start = antinode;
        }
        start = self.b.clone();
        loop {
            let antinode = start.sub(&gcd_diff);
            if antinode.inbounds(max) {
                antinodes.insert(antinode.clone());
            } else {
                break;
            }
            start = antinode;
        }
        return antinodes;
    }
}
