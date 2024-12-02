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

    let reports: Vec<Vec<i32>> = values
        .iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    part_1(reports.clone());
    part_2(reports);
}

fn part_1(reports: Vec<Vec<i32>>) {
    let ans = reports.into_iter().filter(|report| is_valid(&report)).count();
    println!("Total value is: {ans}");
}

fn part_2(reports: Vec<Vec<i32>>) {
    let mut ans = 0;
    for report in reports.iter() {
        if is_valid(report) {
            ans += 1
        } else {
            for i in 0..report.len() {
                let mut subset = report.clone();
                subset.remove(i);
                if is_valid(&subset) {
                    ans += 1;
                    break;
                }

            }
        }
    }
    println!("Total value is: {ans}");
        
}

fn is_valid(report: &Vec<i32>) -> bool {
    let mut diffs: Vec<i32> = Vec::new();
    for window in report.windows(2) {
        diffs.push(window[0] - window[1]);
    }
    return diffs.iter().all(|&x| x >= 1 && x <= 3) || diffs.iter().all(|&x| x <= -1 && x >= -3);
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
