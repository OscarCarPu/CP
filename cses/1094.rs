#![allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str::FromStr;

struct Scanner<R> {
    reader: R,
    tokens: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            tokens: Vec::new(),
        }
    }

    fn next<T: FromStr>(&mut self) -> T {
        while self.tokens.is_empty() {
            let mut line = String::new();
            self.reader
                .read_line(&mut line)
                .expect("Failed to read line");
            self.tokens = line
                .split_whitespace()
                .rev()
                .map(|s| s.to_string())
                .collect();
        }
        self.tokens
            .pop()
            .expect("No more tokens")
            .parse()
            .ok()
            .expect("Failed to parse")
    }
}

fn solve(sc: &mut Scanner<io::StdinLock>) {
    let t: i64 = sc.next();

    let mut bigger: i64 = 0;
    let mut sol: i64 = 0;

    for _ in 0..t {
        let n: i64 = sc.next();
        if n < bigger {
            sol += bigger - n;
        } else {
            bigger = n;
        }
    }
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
