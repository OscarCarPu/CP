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
    let mut v: Vec<i64> = vec![0; n as usize];
    for i in 0..n {
        v[i as usize] = sc.next();
    }
    let mut sol: i64 = i64::MAX;
    fn sub_solve(mut sol: &mut i64, v: &Vec<i64>, mut a: i64, mut b: i64, i: usize) {
        if i == v.len() {
            *sol = min(*sol, (a - b).abs());
            return;
        }
        a += v[i];
        sub_solve(&mut sol, v, a, b, i + 1);
        a -= v[i];
        b += v[i];
        sub_solve(&mut sol, v, a, b, i + 1);
    }
    sub_solve(&mut sol, &v, 0, 0, 0);
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
