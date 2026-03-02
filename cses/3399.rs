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
    let t: i64 = sc.next();
    for _ in 0..t {
        let (n, a, b): (i64, i64, i64) = (sc.next(), sc.next(), sc.next());
        let remaining = n - a - b;
        if remaining < 0 || (a == 0 && b != 0) || (b == 0 && a != 0) {
            println!("NO");
            continue;
        }
        println!("YES");
        let (mut sola, mut solb) = (Vec::new(), Vec::new());

        for i in 0..remaining {
            sola.push(n - i);
            solb.push(n - i);
        }
        for i in 0..a {
            sola.push(n - remaining - i);
            solb.push(n - remaining - i - b);
        }
        for i in 0..b {
            sola.push(n - remaining - i - a);
            solb.push(n - remaining - i);
        }
        for i in sola {
            print!("{} ", i);
        }
        println!("");
        for i in solb {
            print!("{} ", i);
        }
        println!("");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
