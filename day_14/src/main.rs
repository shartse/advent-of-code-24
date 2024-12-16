use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let robots: Vec<Robot> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|s| Robot::new(&s))
        .collect();

    let grid = Grid {
        width: 101,
        height: 103,
    };
    solve(robots, grid);
}

fn solve(robots: Vec<Robot>, grid: Grid) {
    let mut robots = robots.clone();
    let mut i = 1;
    loop {
        for r in robots.iter_mut() {
            r.step(&grid);
        }
        if i == 100 {
            let ans = grid.score(&robots);
            println!("Part 1 answer is {:?}", ans);
        }
        if grid.treelike(&robots) {
            grid.plot(&robots);
            println!("Part 2 answer is {:?}", i);
            break;
        }
        i += 1;
    }
}

#[derive(Debug)]
struct Grid {
    width: i32,
    height: i32,
}

impl Grid {
    fn treelike(&self, robots: &Vec<Robot>) -> bool {
        let mut max_in_place = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut matched = 0;
                for r in robots.iter() {
                    if r.point.0 == x && r.point.1 == y {
                        matched += 1;
                        if matched > max_in_place {
                            max_in_place = matched
                        }
                    }
                }
            }
        }
        return max_in_place == 1;
    }

    fn score(&self, robots: &Vec<Robot>) -> usize {
        let mut total = 1;
        for qx in 0..2 {
            for qy in 0..2 {
                let mut quadrant_matched = 0;
                for y in qy * (self.height / 2 + 1)..((self.height / 2) * (qy + 1) + qy) {
                    for x in qx * (self.width / 2 + 1)..((self.width / 2) * (qx + 1) + qx) {
                        let mut matched = 0;
                        for r in robots.iter() {
                            if r.point.0 == x && r.point.1 == y {
                                matched += 1;
                            }
                        }
                        if matched != 0 {
                            quadrant_matched += matched;
                        }
                    }
                }
                total *= quadrant_matched;
            }
        }
        return total;
    }

    fn plot(&self, robots: &Vec<Robot>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut matched = 0;
                for r in robots.iter() {
                    if r.point.0 == x && r.point.1 == y {
                        matched += 1;
                    }
                }
                if matched == 0 {
                    print!(".");
                } else {
                    print!("{matched}");
                }
            }
            println!("");
        }
        println!("");
    }
}
#[derive(Debug, Clone)]
struct Point(i32, i32);
#[derive(Debug, Clone)]
struct Robot {
    point: Point,
    velocity: Point,
}

impl Robot {
    fn new(line: &str) -> Self {
        let (p, v) = line.split_once(' ').unwrap();
        let (px, py) = p.strip_prefix("p=").unwrap().split_once(',').unwrap();
        let (vx, vy) = v.strip_prefix("v=").unwrap().split_once(',').unwrap();

        let point = Point(px.parse().unwrap(), py.parse().unwrap());
        let velocity = Point(vx.parse().unwrap(), vy.parse().unwrap());
        return Robot { point, velocity };
    }

    fn step(&mut self, grid: &Grid) {
        let mut new_x = self.point.0 + self.velocity.0;
        if new_x < 0 {
            new_x = grid.width + new_x;
        }
        if new_x > grid.width - 1 {
            new_x = new_x - grid.width
        }

        let mut new_y = self.point.1 + self.velocity.1;
        if new_y < 0 {
            new_y = grid.height + new_y;
        }
        if new_y > grid.height - 1 {
            new_y = new_y - grid.height
        }

        self.point.0 = new_x;
        self.point.1 = new_y;
    }
}