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

fn solve(sc: &mut Scanner<io::StdinLock>) {
    let n: i64 = sc.next();
    let mut sol = vec![0, 1];
    for _ in 2..=n {
        let reversed: Vec<_> = sol.iter().rev().cloned().collect();
        sol.extend(reversed);
        for j in 0..sol.len() {
            sol[j] = sol[j] << 1;
            if j < sol.len() / 2 {
                sol[j] += 1;
            }
        }
    }
    for i in 0..sol.len() {
        println!("{}", format!("{:0len$b}", sol[i], len = n as usize));
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
