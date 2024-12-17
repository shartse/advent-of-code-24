use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let (grid, moves) = Grid::new(values);
    solve(grid, moves);
}

fn solve(grid: Grid, moves: Vec<Move>) {
    let mut grid = grid.clone();
    for m in moves.iter() {
        grid.step(&m);
    }
    grid.plot();
    let ans = grid.score();

    println!("Total value is: {ans}");
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}
impl Move {
    fn new(c: &char) -> Self {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("invalid move"),
        }
    }
}

#[derive(Debug, Hash, Clone)]
enum Cell {
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos(usize, usize);

impl Pos {
    fn step(&self, m: &Move) -> Self {
        match m {
            Move::Up => Pos(self.0 - 1, self.1),
            Move::Down => Pos(self.0 + 1, self.1),
            Move::Left => Pos(self.0, self.1 - 1),
            Move::Right => Pos(self.0, self.1 + 1),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: HashMap<Pos, Cell>,
    max: Pos,
    robot: Pos,
}

impl Grid {
    fn score(&self) -> usize {
        let mut ans = 0;
        for (pos, cell) in self.cells.iter() {
            match cell {
                Cell::BoxLeft => ans += (100 * pos.0) + pos.1,
                _ => (),
            }
        }
        return ans;
    }

    fn step(&mut self, m: &Move) {
        let next = self.robot.step(m);
        if let Some(object) = self.cells.get(&next) {
            match object {
                Cell::Wall => return,
                Cell::BoxLeft | Cell::BoxRight => {
                    let pair = self.pair(&next, &object);
                    if self.push_box(&pair, m) {
                        self.robot = next;
                    }
                }
            }
        } else {
            self.robot = next;
        }
    }

    fn push_box(&mut self, pair: &(Pos, Pos), m: &Move) -> bool {
        let (next_left, next_right) = (pair.0.step(m), pair.1.step(m));
        let right_object = self.cells.get(&next_right);
        let left_object = self.cells.get(&next_left);
        match m {
            Move::Down | Move::Up => match (left_object, right_object) {
                (None, None) => {
                    self.move_box(pair, &(next_left, next_right));
                    return true;
                }
                (None, Some(r)) => match r {
                    Cell::Wall => return false,
                    Cell::BoxLeft | Cell::BoxRight => {
                        let next_pair = self.pair(&next_right, &r);
                        if self.push_box(&next_pair, m) {
                            self.move_box(pair, &(next_left, next_right));
                            return true;
                        } else {
                            return false;
                        }
                    }
                },
                (Some(l), None) => match l {
                    Cell::Wall => return false,
                    Cell::BoxLeft | Cell::BoxRight => {
                        let next_pair = self.pair(&next_left, &l);
                        if self.push_box(&next_pair, m) {
                            self.move_box(pair, &(next_left, next_right));
                            return true;
                        } else {
                            return false;
                        }
                    }
                },
                (Some(l), Some(r)) => match (l, r) {
                    (Cell::Wall, _) | (_, Cell::Wall) => return false,
                    (Cell::BoxLeft, Cell::BoxRight) => {
                        let next_pair = (next_left, next_right);
                        if self.push_box(&next_pair, m) {
                            self.move_box(pair, &next_pair);
                            return true;
                        } else {
                            return false;
                        }
                    }
                    (Cell::BoxRight, Cell::BoxLeft) => {
                        let left_pair = self.pair(&next_left, l);
                        let right_pair = self.pair(&next_right, r);
                        let mut test_grid = self.clone();
                        if test_grid.push_box(&left_pair, m) && test_grid.push_box(&right_pair, m) {
                            self.push_box(&left_pair, m);
                            self.push_box(&right_pair, m);
                            self.move_box(pair, &(next_left, next_right));
                            return true;
                        } else {
                            return false;
                        }
                    }
                    _ => panic!("not implemented"),
                },
            },
            Move::Left => {
                if let Some(object) = left_object {
                    match object {
                        Cell::BoxLeft => {
                            panic!("should not hit a left box going left")
                        }
                        Cell::BoxRight => {
                            let next_pair = self.pair(&next_left, &object);
                            if self.push_box(&next_pair, m) {
                                self.move_box(pair, &(next_left, next_right));
                                return true;
                            } else {
                                return false;
                            }
                        }
                        Cell::Wall => return false,
                    }
                } else {
                    self.move_box(pair, &(next_left, next_right));
                    return true;
                }
            }
            Move::Right => {
                if let Some(object) = right_object {
                    match object {
                        Cell::BoxRight => {
                            panic!("should not hit a right box going right")
                        }
                        Cell::BoxLeft => {
                            let next_pair = self.pair(&next_right, &object);
                            if self.push_box(&next_pair, m) {
                                self.move_box(pair, &(next_left, next_right));
                                return true;
                            } else {
                                return false;
                            }
                        }
                        Cell::Wall => return false,
                    }
                } else {
                    self.move_box(pair, &(next_left, next_right));
                    return true;
                }
            }
        }
    }

    fn move_box(&mut self, pair: &(Pos, Pos), next: &(Pos, Pos)) -> bool {
        let a = self.cells.get(&pair.0).unwrap().clone();
        let b = self.cells.get(&pair.1).unwrap().clone();
        self.cells.remove(&pair.0);
        self.cells.remove(&pair.1);
        self.cells.insert(next.0.clone(), a);
        self.cells.insert(next.1.clone(), b);
        return true;
    }

    fn pair(&self, pos: &Pos, cell: &Cell) -> (Pos, Pos) {
        match cell {
            Cell::Wall => panic!("not a box"),
            Cell::BoxLeft => (pos.clone(), Pos(pos.0, pos.1 + 1)),
            Cell::BoxRight => (Pos(pos.0, pos.1 - 1), pos.clone()),
        }
    }

    fn plot(&self) {
        for x in 0..self.max.0 {
            for y in 0..self.max.1 {
                if let Some(cell) = self.cells.get(&Pos(x, y)) {
                    match cell {
                        Cell::Wall => print!("#"),
                        Cell::BoxLeft => print!("["),
                        Cell::BoxRight => print!("]"),
                    }
                } else {
                    if self.robot == Pos(x, y) {
                        print!("@");
                    } else {
                        print!(".");
                    }
                }
            }
            println!("");
        }
        println!("");
    }

    fn new(values: Vec<String>) -> (Self, Vec<Move>) {
        let mut cells = HashMap::new();
        let mut moves = Vec::new();
        let mut robot = Pos(0, 0);
        let max_row = values.iter().position(|l| l.len() == 0).unwrap();
        let max_col = values[0].len();
        for x in 0..max_row {
            let row = &values[x];
            for (y, col) in row.chars().enumerate() {
                let pos = Pos(x, y * 2);
                if col == 'O' {
                    cells.insert(Pos(pos.0, pos.1 + 1), Cell::BoxRight);
                    cells.insert(pos, Cell::BoxLeft);
                } else if col == '#' {
                    cells.insert(Pos(pos.0, pos.1 + 1), Cell::Wall);
                    cells.insert(pos, Cell::Wall);
                } else if col == '@' {
                    robot = pos.clone();
                }
            }
        }
        let max = Pos(max_row, max_col * 2);
        for l in values[max_row..].iter() {
            for c in l.chars() {
                moves.push(Move::new(&c));
            }
        }
        (Grid { cells, max, robot }, moves)
    }
}