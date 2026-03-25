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
    let (x, n) = (sc.next(), sc.next());
    let mut v: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut dist = BTreeMap::new();
    v.insert(0, (usize::MAX, x));
    v.insert(x, (x, usize::MAX));
    dist.insert(x, 1);
    for _ in 0..n {
        let p = sc.next();
        let (lb, ub): (usize, usize) = (
            *v.range(..p).next_back().unwrap().0,
            *v.range((p + 1)..).next().unwrap().0,
        );
        *dist.get_mut(&(ub - lb)).unwrap() -= 1;
        if dist.get(&(ub - lb)).unwrap() == &0 {
            dist.remove(&(ub - lb));
        }
        v.insert(p, (p - lb, ub - p));
        v.entry(lb).and_modify(|(_, r)| *r = p - lb);
        v.entry(ub).and_modify(|(l, _)| *l = ub - p);
        *dist.entry(p - lb).or_insert(0) += 1;
        *dist.entry(ub - p).or_insert(0) += 1;
        print!("{} ", dist.last_key_value().unwrap().0);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
