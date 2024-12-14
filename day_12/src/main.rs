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

    let grid = Grid::new(&values);
    part_1(&grid);
}

fn part_1(grid: &Grid) {
    let mut ans_1 = 0;
    let mut ans_2 = 0;
    let mut visited = HashSet::new();
    let mut start = Pos(0, 0);

    loop {
        let region = grid.region(&start);
        for pos in region.iter() {
            visited.insert(pos.clone());
        }
        ans_1 += grid.price_part_1(&region);
        ans_2 += grid.price_part_2(&region);
        if let Some(next) = grid.next_unvisited(&start, &visited) {
            start = next;
        } else {
            break;
        }
    }

    println!("Part 1 - Total value is: {ans_2}");
    println!("Part 2 - Total value is: {ans_2}");
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos(i32, i32);
#[derive(Debug, Clone)]
struct Grid {
    max_pos: Pos,
    cells: HashMap<Pos, char>,
    kinds: HashSet<char>,
}

impl Grid {
    fn new(lines: &Vec<String>) -> Self {
        let max_pos = Pos((lines.len() - 1) as i32, (lines[0].len() - 1) as i32);
        let mut cells = HashMap::new();
        let mut kinds = HashSet::new();
        for (x, row) in lines.iter().enumerate() {
            for (y, cell) in row.chars().enumerate() {
                cells.insert(Pos(x as i32, y as i32), cell);
                kinds.insert(cell);
            }
        }
        Grid {
            max_pos,
            cells,
            kinds,
        }
    }

    // the price of fence required for a region is found by multiplying that region's area by its perimeter.
    fn price_part_1(&self, region: &HashSet<Pos>) -> usize {
        let area = region.len();
        let mut perimeter = 0;
        for pos in region.iter() {
            perimeter += self.count_fences(pos);
        }
        area * perimeter
    }

    // instead of using the perimeter to calculate the price, you need to use the number of sides
    fn price_part_2(&self, region: &HashSet<Pos>) -> usize {
        let area = region.len();
        let mut sides = 0;
        for pos in region.iter() {
            sides += self.count_corners(pos)
        }
        area * sides
    }

    fn region(&self, start_pos: &Pos) -> HashSet<Pos> {
        let kind = self.cells.get(start_pos).unwrap();
        let mut region = HashSet::new();
        let mut start = HashSet::new();
        start.insert(start_pos.clone());
        let mut visited = HashSet::new();
        loop {
            let mut next_batch = HashSet::new();
            for pos in start.into_iter() {
                visited.insert(pos.clone());
                region.insert(pos.clone());
                let neighbors = self.region_neighbors(&pos);
                for neighbor in neighbors.iter() {
                    if !visited.contains(neighbor) {
                        next_batch.insert(neighbor.clone());
                    }
                }
            }
            if next_batch.is_empty() {
                break;
            }
            start = next_batch;
        }
        return region;
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.0 <= self.max_pos.0 && pos.1 >= 0 && pos.1 <= self.max_pos.1
    }

    fn next_unvisited(&self, pos: &Pos, visited: &HashSet<Pos>) -> Option<Pos> {
        let mut cur_pos = pos.clone();
        loop {
            let next = if cur_pos.0 < self.max_pos.0 {
                Some(Pos(cur_pos.0 + 1, cur_pos.1))
            } else if cur_pos.1 < self.max_pos.1 {
                Some(Pos(0, cur_pos.1 + 1))
            } else {
                None
            };
            match next {
                Some(p) => {
                    if !visited.contains(&p) {
                        return Some(p);
                    } else {
                        cur_pos = p.clone();
                    }
                }
                None => return None,
            }
        }
    }

    fn neighbors(&self, start_pos: &Pos) -> Vec<Pos> {
        return vec![
            Pos(start_pos.0 - 1, start_pos.1),
            Pos(start_pos.0 + 1, start_pos.1),
            Pos(start_pos.0, start_pos.1 - 1),
            Pos(start_pos.0, start_pos.1 + 1),
        ];
    }

    fn count_corners(&self, start_pos: &Pos) -> usize {
        let kind = self.cells.get(start_pos).unwrap();

        let up = self.empty(&Pos(start_pos.0 - 1, start_pos.1), kind);
        let up_left = self.empty(&Pos(start_pos.0 - 1, start_pos.1 - 1), kind);
        let up_right = self.empty(&Pos(start_pos.0 - 1, start_pos.1 + 1), kind);
        let down = self.empty(&Pos(start_pos.0 + 1, start_pos.1), kind);
        let down_left = self.empty(&Pos(start_pos.0 + 1, start_pos.1 - 1), kind);
        let down_right = self.empty(&Pos(start_pos.0 + 1, start_pos.1 + 1), kind);
        let left = self.empty(&Pos(start_pos.0, start_pos.1 - 1), kind);
        let right = self.empty(&Pos(start_pos.0, start_pos.1 + 1), kind);

        let a = left && up || (!left && !up && up_left);
        let b = right && up || (!right && !up && up_right);
        let c = right && down || (!right && !down && down_right);
        let d = left && down || (!left && !down && down_left);

        let corners = vec![a, b, c, d];
        let count = corners.iter().filter(|&x| *x).count();
        return count;
    }

    fn empty(&self, pos: &Pos, kind: &char) -> bool {
        !self.in_bounds(pos) || self.cells.get(pos).unwrap() != kind
    }

    fn count_fences(&self, start_pos: &Pos) -> usize {
        let kind = self.cells.get(start_pos).unwrap();
        let dirs = self.neighbors(start_pos);
        let non_region_neighbors: Vec<Pos> = dirs
            .into_iter()
            .filter(|d| !self.in_bounds(d) || self.cells.get(d).unwrap() != kind)
            .collect();
        non_region_neighbors.len()
    }

    fn region_neighbors(&self, start_pos: &Pos) -> Vec<Pos> {
        let kind = self.cells.get(start_pos).unwrap();
        let dirs = self.neighbors(start_pos);
        let region_neighbors: Vec<Pos> = dirs
            .into_iter()
            .filter(|d| self.in_bounds(d) && self.cells.get(d).unwrap() == kind)
            .collect();
        region_neighbors
    }
}
