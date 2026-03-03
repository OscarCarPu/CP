#![allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};
use std::str::FromStr;

#[allow(unused)]
const MOD: usize = 1_000_000_007;

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
fn pow_mod(mut base: usize, mut exp: usize) -> usize {
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

#[allow(unused)]
fn mex(set: &HashSet<usize>) -> usize {
    (0..).find(|i| !set.contains(i)).unwrap()
}

fn solve(sc: &mut Scanner<io::StdinLock>) {
    let (n, m, k): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let mut apartments: Vec<usize> = Vec::new();
    let mut applicants: Vec<usize> = Vec::new();
    for _ in 0..n {
        let a: usize = sc.next();
        applicants.push(a);
    }
    for _ in 0..m {
        let a: usize = sc.next();
        apartments.push(a);
    }
    applicants.sort();
    apartments.sort();
    let (mut idx_app, mut idx_ap): (usize, usize) = (0, 0);
    let mut sol: usize = 0;
    while idx_app < applicants.len() && idx_ap < apartments.len() {
        while apartments[idx_ap] + k < applicants[idx_app] {
            idx_ap += 1;
            if idx_ap == apartments.len() {
                break;
            }
        }
        if idx_ap == apartments.len() {
            break;
        }
        if apartments[idx_ap] <= applicants[idx_app] + k {
            sol += 1;
            idx_ap += 1;
        }
        idx_app += 1;
    }
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
