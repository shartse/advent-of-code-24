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
    let mut ans = 0;
    println!("Total value is: {ans}");
}

fn part_2(vals: Vec<String>) {
    let mut ans = 0;
    println!("Total value is: {ans}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn part_2() {
        panic!("Make this test fail");
    }
}
