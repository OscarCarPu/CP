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
    let n = sc.next();
    let mut freq: HashMap<isize, usize> = HashMap::new();
    freq.insert(0, 1);
    let (mut prefix, mut sol): (isize, usize) = (0, 0);
    for _ in 0..n {
        let x: isize = sc.next();
        prefix += x;

        let rem = ((prefix % n) + n) % n;
        sol += freq.get(&rem).unwrap_or(&0);
        *freq.entry(rem).or_insert(0) += 1;
    }
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    solve(&mut scanner);
}
