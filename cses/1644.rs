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
    let (n, a, b): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let mut v: Vec<isize> = Vec::new();
    v.push(0);
    let mut sum: isize = 0;
    for _ in 0..n {
        let x: isize = sc.next();
        sum += x;
        v.push(sum);
    }

    let mut dq: VecDeque<usize> = VecDeque::new();
    let mut sol: isize = isize::MIN;

    for i in a..=n {
        let r = i - a;
        while let Some(&l) = dq.back() {
            if v[l] >= v[r] {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(r);

        if i >= b {
            let lo = i - b;
            while let Some(&front) = dq.front() {
                if front < lo {
                    dq.pop_front();
                } else {
                    break;
                }
            }
        }

        let j = *dq.front().unwrap();
        let candidate = v[i] - v[j];
        sol = max(sol, candidate);
    }
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
