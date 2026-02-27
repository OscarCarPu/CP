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

    for _ in 0..t {
        let x: i64 = sc.next();
        let y: i64 = sc.next();
        if max(x, y) % 2 == 0 {
            if x >= y {
                println!("{}", x * x - y + 1);
                continue;
            } else {
                println!("{}", (y - 1) * (y - 1) + x);
                continue;
            }
        } else {
            if x >= y {
                println!("{}", (x - 1) * (x - 1) + y);
                continue;
            } else {
                println!("{}", y * y - x + 1);
                continue;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
