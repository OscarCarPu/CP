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
    let n = sc.next();
    let mut l = 0;
    let mut v: VecDeque<usize> = VecDeque::new();
    let mut set = HashSet::new();
    let mut sol: u64 = 0;
    for r in 0..n {
        let x = sc.next();
        while set.contains(&x) {
            set.remove(&v.pop_back().unwrap());
            l += 1;
        }
        v.push_front(x);
        set.insert(x);
        sol += r - l + 1;
    }
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    solve(&mut scanner);
}
