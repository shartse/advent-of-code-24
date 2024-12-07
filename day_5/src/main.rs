use std::{
    cmp::Ordering,
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

    let split = values.iter().position(|n| n == "").unwrap();
    let (rules, jobs) = values.split_at(split);

    let mut rule_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for rule in rules {
        let (before, after) = rule.split_once('|').unwrap();
        if let Some(afters) = rule_map.get_mut(before) {
            afters.insert(after);
        } else {
            let mut afters = HashSet::new();
            afters.insert(after);
            rule_map.insert(before, afters);
        }
    }
    part_1(jobs, rule_map.clone());
    part_2(jobs, rule_map);
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Page<'a> {
    v: &'a str,
    after: Option<HashSet<&'a str>>,
}

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Page) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page<'_> {
    fn cmp(&self, other: &Page) -> Ordering {
        if let Some(after) = self.after.clone() {
            if after.contains(&other.v) {
                return Ordering::Greater;
            }
        }
        if let Some(after) = other.after.clone() {
            if after.contains(&self.v) {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

fn part_1(jobs: &[String], rule_map: HashMap<&str, HashSet<&str>>) {
    let mut ans = 0;
    for job in jobs.iter().filter(|&j| j.len() > 0) {
        let pages: Vec<Page> = job
            .split(',')
            .map(|c| Page {
                v: c,
                after: rule_map.clone().get(c).cloned(),
            })
            .collect();
        let mut sorted = pages.clone();
        sorted.sort();
        sorted.reverse();
        if sorted.eq(&pages) {
            let middle = pages.len() / 2;
            ans += pages[middle].v.parse::<i32>().unwrap();
        }
    }
    println!("Total value is: {ans}");
}

fn part_2(jobs: &[String], rule_map: HashMap<&str, HashSet<&str>>) {
    let mut ans = 0;
    for job in jobs {
        let pages: Vec<Page> = job
            .split(',')
            .map(|c| Page {
                v: c,
                after: rule_map.clone().get(c).cloned(),
            })
            .collect();
        let mut sorted = pages.clone();
        sorted.sort();
        if sorted != pages {
            let middle = pages.len() / 2;
            ans += pages[middle].v.parse::<i32>().unwrap();
        }
    }
    println!("Total value is: {ans}");
}
