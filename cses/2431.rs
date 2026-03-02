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
    let t: usize = sc.next();

    for _ in 0..t {
        let mut n: usize = sc.next();
        let mut digits: usize = 1;
        let mut count_digits: usize = 9;
        while n > count_digits {
            digits += 1;
            count_digits += 10_usize.pow(digits as u32 - 1) * 9 * digits;
        }
        n -= count_digits - 10_usize.pow(digits as u32 - 1) * 9 * digits;
        let mut base: usize = 0;
        for i in 0..digits - 1 {
            base += 10_usize.pow(i as u32) * 9;
        }
        let sol: usize = base + (n + digits - 1) / digits;
        let mut pos: usize = n % digits;
        if pos == 0 {
            pos = digits;
        }
        println!("{}", sol.to_string().chars().nth(pos - 1).unwrap());
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
