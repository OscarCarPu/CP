#![allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
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
    let n: usize = sc.next();
    let mut v: VecDeque<usize> = VecDeque::new();
    for i in 1..=n {
        v.push_back(i);
    }
    let mut extract: bool = false;
    while !v.is_empty() {
        let x = v.pop_front().unwrap();
        if extract {
            print!("{} ", x);
        } else {
            v.push_back(x);
        }
        extract = !extract;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    solve(&mut scanner);
}
