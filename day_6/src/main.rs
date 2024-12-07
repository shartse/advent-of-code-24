use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("test.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let map = Map::new(values.clone());
    part_1(map.clone());
    part_2(map);
}

fn part_1(map: Map) {
    println!("Part 1 total value is: {:?}", map.count_steps().unwrap());
}

fn part_2(map: Map) {
    let mut ans = 0;
    let cells = map.grid.keys().clone();
    for each in cells {
        let mut new_map = map.clone();
        *new_map.grid.get_mut(each).unwrap() = Cell::Obstruction;
        if new_map.count_steps().is_none() {
            ans += 1;
        }
    }
    println!("Part 2 total value is: {ans}");
}

#[derive(Clone, Debug)]
struct Map {
    pos: (i32, i32),
    orientation: Orientation,
    grid: HashMap<(i32, i32), Cell>,
}

impl Map {
    fn new(input: Vec<String>) -> Self {
        let orientation = Orientation::Up;
        let mut pos = (0 as i32, 0 as i32);
        let mut grid = HashMap::new();
        for (x, row) in input.iter().enumerate() {
            for (y, cell) in row.chars().enumerate() {
                if cell == '^' {
                    pos = (x as i32, y as i32);
                }
                if cell == '#' {
                    grid.insert((x as i32, y as i32), Cell::Obstruction);
                } else {
                    grid.insert((x as i32, y as i32), Cell::Empty);
                }
            }
        }
        return Map {
            pos,
            orientation,
            grid,
        };
    }

    fn step(&mut self) -> Option<(i32, i32)> {
        let next_pos = match self.orientation {
            Orientation::Up => (self.pos.0 - 1, self.pos.1),
            Orientation::Down => (self.pos.0 + 1, self.pos.1),
            Orientation::Left => (self.pos.0, self.pos.1 - 1),
            Orientation::Right => (self.pos.0, self.pos.1 + 1),
        };
        match self.grid.get(&next_pos) {
            Some(cell) => match cell {
                Cell::Obstruction => {
                    self.orientation.turn_right();
                    return Some(self.pos);
                }
                Cell::Empty => {
                    self.pos = next_pos;
                    return Some(next_pos);
                }
            },
            None => return None,
        }
    }

    fn count_steps(self) -> Option<usize> {
        let start_pos = self.pos.clone();
        let start_orientatin = self.orientation.clone();
        let mut walking_map = self.clone();
        let mut visited: HashMap<(i32, i32), Vec<Orientation>> = HashMap::new();
        visited.insert(start_pos, vec![start_orientatin]);
        loop {
            match walking_map.step() {
                Some(pos) => {
                    if let Some(visits) = visited.get_mut(&pos) {
                        if visits.contains(&walking_map.orientation) {
                            return None;
                        }
                    } else {
                        visited.insert(pos.clone(), vec![walking_map.orientation.clone()]);
                    }
                }
                None => {
                    break;
                }
            }
        }
        return Some(visited.keys().len());
    }
}

#[derive(Clone, Debug)]
enum Cell {
    Obstruction,
    Empty,
}

#[derive(Clone, Debug, PartialEq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn turn_right(&mut self) {
        *self = match self {
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
        }
    }
}
