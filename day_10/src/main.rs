use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    solve(&values);
}

fn solve(vals: &Vec<String>) {
    let map = TopoMap::new(vals);
    let trailheads = map.trailheads();

    let mut part_1 = 0;
    let mut part_2 = 0;
    for trailhead in trailheads.iter() {
        let (a, b) = map.trailhead_score(trailhead);
        part_1 +=a;
        part_2+= b;
    }
    println!("Part 1 is {part_1}, Part 2 is {part_2}");
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct TopoMap {
    grid: HashMap<Pos, u32>,
    max: Pos,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos(usize, usize);

impl TopoMap {
    fn new(lines: &Vec<String>) -> Self {
        let mut grid = HashMap::new();
        let max_row = lines.len();
        let max_col = lines[0].len();
        for (x, row) in lines.iter().enumerate() {
            for (y, col) in row.chars().enumerate() {
                grid.insert(Pos(x, y), col.to_digit(10).unwrap());
            }
        }
        TopoMap {
            grid,
            max: Pos(max_row, max_col),
        }
    }

    fn trailheads(&self) -> Vec<Pos> {
        let mut trailheads = Vec::new();
        for (pos, elevation) in self.grid.iter() {
            if *elevation == 0 {
                trailheads.push(pos.clone());
            }
        }
        return trailheads;
    }

    fn trailhead_score(&self, trailhead_start: &Pos) -> (usize, usize) {
        let mut ways_reached = Vec::new();
        let mut unique_peaks_reached = HashSet::new();
        let mut starts = Vec::new();
        let directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        starts.push(trailhead_start.clone());

        loop {
            if starts.is_empty() {
                break;
            }
            let mut next_starts = Vec::new();
            for pos in starts.iter() {
                let start_elevation = self.elevation(pos);
                for dir in directions.iter() {
                    if let Some((next, next_elevation)) = self.climb(pos, start_elevation, dir) {
                        if next_elevation == 9 {
                            ways_reached.push(next.clone());
                            unique_peaks_reached.insert(next);
                        } else {
                            next_starts.push(next);
                        }
                    }
                }
            }
            starts = next_starts;
        }
        return (unique_peaks_reached.len(), ways_reached.len())
    }

    fn elevation(&self, pos: &Pos) -> u32 {
        self.grid.get(pos).unwrap().to_owned()
    }

    fn climb(
        &self,
        start: &Pos,
        start_elevation: u32,
        direction: &Direction,
    ) -> Option<(Pos, u32)> {
        let next = match direction {
            &Direction::Up => {
                if start.0 > 0 {
                    Option::Some(Pos(start.0 - 1, start.1))
                } else {
                    Option::None
                }
            }
            &Direction::Down => {
                if start.0 < self.max.0 - 1 {
                    Option::Some(Pos(start.0 + 1, start.1))
                } else {
                    Option::None
                }
            }
            &Direction::Left => {
                if start.1 > 0 {
                    Option::Some(Pos(start.0, start.1 - 1))
                } else {
                    Option::None
                }
            }
            &Direction::Right => {
                if start.1 < self.max.1 - 1 {
                    Option::Some(Pos(start.0, start.1 + 1))
                } else {
                    Option::None
                }
            }
        };

        if let Some(next) = next {
            let next_elevation = self.elevation(&next);
            if next_elevation == start_elevation + 1 {
                return Some((next, next_elevation));
            }
            return None;
        } else {
            return None;
        }
    }
}