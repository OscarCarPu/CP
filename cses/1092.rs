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
    let n: i64 = sc.next();
    let mut set_a = HashSet::new();
    let mut set_b = HashSet::new();
    let mut sum_a = 0;
    let mut sum_b = 0;
    for i in (1..=n).rev() {
        if sum_a > sum_b {
            set_b.insert(i);
            sum_b += i;
        } else {
            set_a.insert(i);
            sum_a += i;
        }
    }
    if sum_a == sum_b {
        println!("YES");
        println!("{}", set_a.len());
        for i in set_a {
            print!("{} ", i);
        }
        println!();
        println!("{}", set_b.len());
        for i in set_b {
            print!("{} ", i);
        }
    } else {
        println!("NO");
    }
}
fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
