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
    let (n, m) = (sc.next(), sc.next());
    let (mut pos, mut cnt) = (vec![0 as usize; n + 1], vec![1 as usize; n + 1]);
    let mut v = Vec::new();
    v.push(0);
    for i in 1..=n {
        let x: usize = sc.next();
        v.push(x);
        pos[x] = i;
        if pos[x - 1] != 0 {
            cnt[x] = 0;
        }
    }
    pos[0] = usize::MAX;
    let mut sol = cnt.iter().sum::<usize>() - 1;
    for _ in 0..m {
        let (a, b): (usize, usize) = (sc.next(), sc.next());
        let (na, nb) = (v[a], v[b]);
        pos.swap(v[a], v[b]);
        v.swap(a, b);
        sol = sol
            - cnt[na]
            - cnt[nb]
            - (if na != n && na + 1 != nb {
                cnt[na + 1]
            } else {
                0
            })
            - (if nb != n && nb + 1 != na {
                cnt[nb + 1]
            } else {
                0
            });
        cnt[na] = (pos[na] < pos[na - 1]) as usize;
        cnt[nb] = (pos[nb] < pos[nb - 1]) as usize;
        if na != n && na + 1 != nb {
            cnt[na + 1] = (pos[na + 1] < pos[na]) as usize;
        }
        if nb != n && nb + 1 != na {
            cnt[nb + 1] = (pos[nb + 1] < pos[nb]) as usize;
        }
        sol = sol
            + cnt[na]
            + cnt[nb]
            + (if na != n && na + 1 != nb {
                cnt[na + 1]
            } else {
                0
            })
            + (if nb != n && nb + 1 != na {
                cnt[nb + 1]
            } else {
                0
            });
        println!("{}", sol);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
