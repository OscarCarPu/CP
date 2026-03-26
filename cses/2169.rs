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
    let n: usize = sc.next();
    let mut ranges: Vec<(i64, i64, usize)> = (0..n)
        .map(|i| {
            let x: i64 = sc.next();
            let y: i64 = sc.next();
            (x, y, i)
        })
        .collect();

    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut sorted_r: Vec<i64> = ranges.iter().map(|&(_, r, _)| r).collect();
    sorted_r.sort_unstable();
    sorted_r.dedup();
    let compress = |r: i64| sorted_r.partition_point(|&x| x < r) + 1; // 1-indexed

    let m = sorted_r.len();
    let mut contains = vec![0i64; n];
    let mut contained = vec![0i64; n];

    let mut fen = vec![0i64; m + 1];
    let fen_update = |fen: &mut Vec<i64>, mut i: usize, delta: i64| {
        while i <= m {
            fen[i] += delta;
            i += i & i.wrapping_neg();
        }
    };
    let fen_query = |fen: &Vec<i64>, mut i: usize| -> i64 {
        let mut s = 0i64;
        while i > 0 {
            s += fen[i];
            i -= i & i.wrapping_neg();
        }
        s
    };

    for &(_, r, idx) in &ranges {
        let cr = compress(r);
        let total = fen_query(&fen, m);
        let below = fen_query(&fen, cr - 1);
        contained[idx] = total - below;
        fen_update(&mut fen, cr, 1);
    }

    let mut fen2 = vec![0i64; m + 1];
    for &(_, r, idx) in ranges.iter().rev() {
        let cr = compress(r);
        contains[idx] = fen_query(&fen2, cr);
        fen_update(&mut fen2, cr, 1);
    }

    for i in 0..n {
        print!("{}{}", contains[i], if i < n - 1 { " " } else { "\n" });
    }
    for i in 0..n {
        print!("{}{}", contained[i], if i < n - 1 { " " } else { "\n" });
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    solve(&mut scanner);
}
