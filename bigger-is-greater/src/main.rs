use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::fs::File;
use std::str;
use std::io;

fn read_line<T: BufRead>(lines: &mut Lines<T>) -> String {
    lines.next().unwrap().unwrap()
}

fn read_int<T: BufRead>(lines: &mut Lines<T>) -> i64 {
    read_line(lines).parse().unwrap()
}

fn main() {
    //    let file_name = "data/tiny-in.txt";
    //    let file_name = "data/A-small-practice.in";
    let file_name = "data/input03.txt";
    let f = File::open(file_name).unwrap();
    let file = BufReader::new(&f);

    //    let stdin = io::stdin();
    //    let file = stdin.lock();

    let mut lines: Lines<_> = file.lines();

    let t = read_int(&mut lines);

    for _ti in 1..(t + 1) {
        let word = read_line(&mut lines);
        println!("{}", solve(word));
    }
}

fn solve(word: String) -> String {
    let mut w = word.into_bytes();
    let i_option = (0..w.len() - 1).rev().find(|&i| w[i] < w[i + 1]);
    if let Some(i) = i_option {
        let wi = w[i];
        let j = (i + 1..w.len()).rev().find(|&j| wi < w[j]).unwrap();
        w[i] = w[j];
        w[j] = wi;
        w[i + 1..].sort();
        unsafe { String::from_utf8_unchecked(w) }
    } else {
        "no answer".to_string()
    }
}
