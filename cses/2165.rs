#![allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::str::FromStr;

#[allow(unused)]
const MOD: i64 = 1_000_000_007;

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

#[allow(unused)]
fn pow_mod(mut base: i64, mut exp: i64) -> i64 {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % MOD;
        }
        exp /= 2;
        base = (base * base) % MOD;
    }
    res
}

fn subsolve(n: i64, source: i64, target: i64, aux: i64) {
    if n == 1 {
        println!("{} {}", source, target);
        return;
    }
    subsolve(n - 1, source, aux, target);
    println!("{} {}", source, target);
    subsolve(n - 1, aux, target, source);
}

fn solve(sc: &mut Scanner<io::StdinLock>) {
    let n: i64 = sc.next();

    println!("{}", (1 << n) - 1);
    subsolve(n, 1, 3, 2);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
