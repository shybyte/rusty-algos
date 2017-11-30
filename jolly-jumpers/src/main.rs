extern crate bit_set;

use std::io;
use std::io::{BufReader, BufRead};
use bit_set::BitSet;

pub fn parse_ints(line: &str) -> Vec<i64> {
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
    for line in BufReader::new(io::stdin()).lines() {
        if solve(&parse_ints(&line.unwrap())) {
            println!("Jolly");
        } else {
            println!("Not jolly");
        }
    }
}

pub fn solve(input: &[i64]) -> bool {
    let mut found_differences = BitSet::new();
    let n = input[0] as usize;
    for pair in input[1..].windows(2) {
        let d = ((pair[1] - pair[0]).abs()) as usize;
        if d < 1 || d >= n || found_differences.contains(d) {
            return false;
        }
        found_differences.insert(d);
    }
    true
}