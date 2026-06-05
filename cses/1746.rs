#![allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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
struct MexSet {
    present: HashSet<usize>,
    absent: BTreeSet<usize>,
    max_val: usize,
}
#[allow(unused)]
impl MexSet {
    fn new(max_val: usize) -> Self {
        Self {
            present: HashSet::new(),
            absent: (0..=max_val).collect(),
            max_val,
        }
    }

    fn insert(&mut self, val: usize) {
        if self.present.insert(val) {
            self.absent.remove(&val);
        }
    }
    fn remove(&mut self, val: usize) {
        if self.present.remove(&val) {
            self.absent.insert(val);
        }
    }
    fn mex(&self) -> usize {
        *self.absent.iter().next().unwrap()
    }
}

#[allow(unused)]
struct Fenwick(Vec<i64>);
#[allow(unused)]
impl Fenwick {
    fn new(n: usize) -> Self {
        let mut f = Self(vec![0; n + 1]);
        for i in 1..=n {
            f.update(i, 1);
        }
        f
    }
    fn update(&mut self, mut i: usize, delta: i64) {
        while i < self.0.len() {
            self.0[i] += delta;
            i += i & i.wrapping_neg();
        }
    }
    fn find_kth(&self, mut k: i64) -> usize {
        let mut pos = 0;
        let mut pw = 1 << self.0.len().ilog2();
        while pw > 0 {
            let next = pos + pw;
            if next < self.0.len() && self.0[next] < k {
                k -= self.0[next];
                pos = next;
            }
            pw >>= 1;
        }
        pos + 1
    }
}

fn solve(sc: &mut Scanner<io::StdinLock>) {
    let (n, m): (usize, usize) = (sc.next(), sc.next());
    let x: Vec<usize> = (0..n).map(|_| sc.next()).collect();
    let mut dp = vec![0usize; m + 2];

    if x[0] == 0 {
        for v in 1..=m {
            dp[v] = 1;
        }
    } else {
        dp[x[0]] = 1;
    }

    let mut next = vec![0usize; m + 2];
    for i in 1..n {
        for v in 1..=m {
            next[v] = dp[v - 1] + dp[v] + dp[v + 1];
            next[v] %= MOD;
        }

        if x[i] == 0 {
            dp.copy_from_slice(&next);
        } else {
            dp.iter_mut().for_each(|d| *d = 0);
            dp[x[i]] = next[x[i]];
        }
        next.iter_mut().for_each(|d| *d = 0);
    }

    let ans = dp[1..=m].iter().fold(0usize, |a, &b| (a + b) % MOD);
    println!("{}", ans);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    solve(&mut scanner);
}
