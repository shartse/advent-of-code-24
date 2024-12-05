use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    part_1(values.clone());
    is_x_of_mas(values);
}

fn part_1(rows: Vec<Vec<char>>) {
    let mut ans = 0;
    let directions: [Direction; 8] = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    for (x, row) in rows.iter().enumerate() {
        for y in 0..row.len() {
            if rows[x][y] == 'X' {
                for dir in directions.iter() {
                    if is_xmas(&rows, x, y, dir) {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("Total value is: {ans}");
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn letter_idx(row: i32, col: i32, letter: i32, direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Left => (row, col - letter),
        Direction::Right => (row, col + letter),
        Direction::Up => (row + letter, col),
        Direction::Down => (row - letter, col),
        Direction::UpLeft => (row - letter, col + letter),
        Direction::UpRight => (row + letter, col + letter),
        Direction::DownLeft => (row - letter, col - letter),
        Direction::DownRight => (row + letter, col - letter),
    }
}

fn is_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize, direction: &Direction) -> bool {
    let letters: [char; 3] = ['M', 'A', 'S'];
    for (idx, val) in letters.iter().enumerate() {
        let letter = idx + 1;
        let m_idx = letter_idx(row as i32, col as i32, letter as i32, direction);
        if m_idx.0 >= 0 && m_idx.1 >= 0 {
            match grid.get(m_idx.0 as usize) {
                Some(row) => match row.get(m_idx.1 as usize) {
                    Some(c) => {
                        if c != val {
                            return false;
                        }
                    }
                    None => return false,
                },
                None => return false,
            }
        } else {
            return false;
        }
    }
    return true;
}

fn is_x_of_mas(rows: Vec<Vec<char>>) {
    let mut ans = 0;
    for (x, row) in rows.iter().enumerate() {
        for y in 0..row.len() {
            if rows[x][y] == 'A' {
                if valid_part2(&rows, x, y) {
                    ans += 1;
                }
            }
        }
    }
    println!("Total value is: {ans}");
}

fn check_loc(grid: &Vec<Vec<char>>, loc: (usize, usize), val: char) -> bool {
    match grid.get(loc.0) {
        Some(row) => match row.get(loc.1) {
            Some(&c) => {
                if c == val {
                    return true;
                } else {
                    return false;
                }
            }
            None => return false,
        },
        None => return false,
    }
}

fn valid_part2(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if row > 0 && col > 0 {
        let top_right = (row + 1, col + 1);
        let bottom_left = (row - 1, col - 1);

        let top_left = (row + 1, col - 1);
        let bottom_right = (row - 1, col + 1);

        let first = (check_loc(grid, top_right, 'S') && check_loc(grid, bottom_left, 'M')
            || check_loc(grid, top_right, 'M') && check_loc(grid, bottom_left, 'S'));
        let second = (check_loc(grid, top_left, 'S') && check_loc(grid, bottom_right, 'M')
            || check_loc(grid, top_left, 'M') && check_loc(grid, bottom_right, 'S'));
        return first && second;
    } else {
        return false;
    }
}