use std::io;
use std::str::FromStr;

pub fn read_line<F: FromStr>() -> F {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().map_err(|_e| format!("Error while parsing '{:?}'", line)).unwrap()
}

pub fn read_ints() -> Vec<i64> {
    let line: String = read_line();
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let _n: i64 = read_line();
    println!("{}", solve(&read_ints()));
}

pub fn solve(candles: &[i64]) ->  usize {
    let max_height = candles.iter().max().unwrap();
    candles.iter().filter(|&height| height == max_height).count()
}