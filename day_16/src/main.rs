use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let maze = Maze::new(&values);
    solve(&maze);
}

fn solve(maze: &Maze) {
    let steps = maze.solve();
    let mut total_cells: HashSet<Pos> = HashSet::new();
    for step in steps {
        for (pos, _) in step.path {
            total_cells.insert(pos);
        }
    }
    let matching = total_cells.len() + 1;
    println!("Part 2: count of cells in valid path is {matching}");
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
enum Cell {
    Wall,
    Empty,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos(usize, usize);
impl Pos {
    fn next(&self, dir: &Dir) -> Self {
        match dir {
            Dir::N => Pos(self.0 - 1, self.1),
            Dir::E => Pos(self.0, self.1 + 1),
            Dir::S => Pos(self.0 + 1, self.1),
            Dir::W => Pos(self.0, self.1 - 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Step {
    pos: Pos,
    dir: Dir,
    score: usize,
    path: HashSet<(Pos, Dir)>,
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Step) -> Ordering {
        self.score.cmp(&other.score)
    }
}

#[derive(Debug)]

struct Maze {
    cells: HashMap<Pos, Cell>,
    start: Pos,
    facing: Dir,
    end: Pos,
}

impl Maze {
    fn solve(&self) -> Vec<Step> {
        let mut min_len: Option<usize> = None;
        let mut min_paths: Vec<Step> = Vec::new();
        let mut heap: BinaryHeap<Reverse<Step>> = BinaryHeap::new();
        let mut visited = HashSet::new();

        let first_step = Step {
            pos: self.start.clone(),
            dir: self.facing.clone(),
            score: 0,
            path: HashSet::new(),
        };

        visited.insert((first_step.pos.clone(), first_step.dir.clone()));
        heap.push(Reverse(first_step));

        loop {
            let next = match heap.pop() {
                Some(rev_step) => rev_step.0,
                None => break,
            };
            let edge = self.steps(&next);
            for (edge_pos, edge_dir) in edge.iter() {
                if !visited.contains(&(edge_pos.clone(), edge_dir.clone())) {
                    let score_add = if edge_dir == &next.dir { 1 } else { 1001 };
                    let mut path = next.path.clone();

                    visited.insert((next.pos.clone(), next.dir.clone()));
                    path.insert((next.pos.clone(), next.dir.clone()));
                    let step = Step {
                        pos: edge_pos.clone(),
                        dir: edge_dir.clone(),
                        score: next.score + score_add,
                        path,
                    };
                    if edge_pos == &self.end {
                        let score = next.score + score_add;
                        if let Some(min) = min_len {
                            if score == min {
                                min_paths.push(step.clone());
                            }
                        } else {
                            min_len = Some(score);
                            min_paths.push(step.clone());
                        }
                    } else {
                        heap.push(Reverse(step));
                    }
                }
            }
        }

        println!(
            "Part 1: found {:?} paths of score {:?}",
            min_paths.len(),
            min_len.unwrap()
        );
        return min_paths;
    }

    fn new(lines: &Vec<String>) -> Self {
        let mut start = Pos(0, 0);
        let mut end = Pos(0, 0);
        let mut cells = HashMap::new();
        for (x, row) in lines.iter().enumerate() {
            for (y, c) in row.chars().enumerate() {
                let pos = Pos(x, y);
                if c == '#' {
                    cells.insert(pos, Cell::Wall);
                } else if c == '.' {
                    cells.insert(pos, Cell::Empty);
                } else if c == 'E' {
                    end = pos.clone();
                    cells.insert(pos, Cell::Empty);
                } else if c == 'S' {
                    start = pos.clone();
                    cells.insert(pos, Cell::Empty);
                }
            }
        }
        let facing = Dir::E;
        return Maze {
            cells,
            start,
            facing,
            end,
        };
    }

    fn steps(&self, step: &Step) -> Vec<(Pos, Dir)> {
        let dirs: Vec<(Pos, Dir)> = match step.dir {
            Dir::N => vec![Dir::N, Dir::W, Dir::E],
            Dir::E => vec![Dir::E, Dir::S, Dir::N],
            Dir::S => vec![Dir::S, Dir::W, Dir::E],
            Dir::W => vec![Dir::W, Dir::S, Dir::N],
        }
        .into_iter()
        .map(|d| {
            let next = (step.pos.next(&d), d);
            next
        })
        .filter(|(p, _)| self.cells.get(p).is_some())
        .filter(|(p, _)| matches!(self.cells.get(p).unwrap(), Cell::Empty))
        .collect();
        return dirs;
    }
}
