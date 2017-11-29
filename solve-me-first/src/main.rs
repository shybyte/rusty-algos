use std::io;
use std::str::FromStr;

pub fn read_line<F: FromStr>() -> F {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().map_err(|_e| format!("Error while parsing '{:?}'", line)).unwrap()
}

fn main() {
    println!("{}", read_line::<i64>() + read_line::<i64>());
}