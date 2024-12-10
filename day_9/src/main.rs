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

    let val = values[0].trim();
    let mut blocks: Vec<Block> = Vec::new();
    for (i, v) in val.chars().enumerate() {
        let num_blocks = v.to_string().parse::<usize>().unwrap();
        if i % 2 == 0 {
            for _ in 0..num_blocks {
                blocks.push(Block::File(i / 2));
            }
        } else {
            for _ in 0..num_blocks {
                blocks.push(Block::Empty);
            }
        }
    }
    part_1(&blocks);
    part_2(&blocks);
}

fn part_1(blocks: &Vec<Block>) {
    let mut blocks = blocks.to_vec();
    loop {
        let first_empty = blocks
            .iter()
            .position(|b| matches!(b, Block::Empty))
            .unwrap();
        let last_filled = (blocks.len() - 1)
            - blocks
                .iter()
                .rev()
                .position(|b| !matches!(b, Block::Empty))
                .unwrap();

        if first_empty > last_filled {
            break;
        }
        blocks.swap(first_empty, last_filled);
    }
    println!("Part 1 value is: {:?}", checksum(&blocks));
}

fn part_2(blocks: &Vec<Block>) {
    let mut blocks = blocks.to_vec();
    let mut file_id = max_file_id(&blocks);
    loop {
        let file_pos = blocks
            .iter()
            .position(|b| match b {
                Block::Empty => false,
                Block::File(id) => *id == file_id,
            })
            .unwrap();
        let file_len = file_len(&blocks, file_id);
        let mut checked_so_far = 0;
        loop {
            let empty_pos = blocks[checked_so_far..]
                .iter()
                .position(|b| matches!(b, Block::Empty))
                .unwrap()
                + checked_so_far;
            let empty_len = empty_len(&blocks, empty_pos);
            if empty_pos >= file_pos {
                break;
            } else if empty_len >= file_len {
                copy_file(&mut blocks, empty_pos, file_pos, file_len);
                break;
            } else {
                checked_so_far = empty_pos + empty_len;
            }
        }
        if file_id == 0 {
            break;
        } else {
            file_id -= 1;
        }
    }
    println!("Part 2 value is: {:?}", checksum(&blocks));
}

fn copy_file(blocks: &mut Vec<Block>, empty_pos: usize, file_pos: usize, file_len: usize) {
    for i in 0..file_len {
        let empty_idx = empty_pos + i;
        let file_idx = file_pos + i;
        blocks.swap(empty_idx, file_idx);
    }
}

fn checksum(blocks: &Vec<Block>) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(i, v)| {
            i * match v {
                Block::Empty => 0,
                Block::File(id) => *id,
            }
        })
        .sum()
}

fn max_file_id(blocks: &Vec<Block>) -> usize {
    blocks
        .iter()
        .map(|b| match b {
            Block::Empty => 0,
            Block::File(id) => *id,
        })
        .max()
        .unwrap()
}

fn file_len(blocks: &Vec<Block>, id: usize) -> usize {
    blocks
        .iter()
        .filter(|b| match b {
            Block::Empty => false,
            Block::File(i) => *i == id,
        })
        .count()
}

fn empty_len(blocks: &Vec<Block>, idx: usize) -> usize {
    let mut len = 0;
    let mut cur_idx = idx;
    loop {
        match blocks[cur_idx] {
            Block::Empty => len += 1,
            Block::File(_) => break,
        }
        cur_idx += 1;
        if cur_idx >= blocks.len() {
            break;
        }
    }
    return len;
}

#[derive(Debug, Clone)]
enum Block {
    Empty,
    File(usize),
}
