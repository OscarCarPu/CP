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
    let mut freq: Vec<usize> = vec![0; ('Z' as usize - 'A' as usize + 1) as usize];
    let s: String = sc.next();
    for c in s.chars() {
        freq[c as usize - 'A' as usize] += 1;
    }
    let mut sol: String = String::new();
    let mut c: char = '3';
    for i in 'A'..='Z' {
        if freq[i as usize - 'A' as usize] > (s.len() + 1) / 2 {
            println!("{}", -1);
            return;
        }
    }
    while sol.len() < s.len() {
        let remaining = s.len() - sol.len();
        let mut found = false;
        for i in 'A'..='Z' {
            let idx = i as usize - 'A' as usize;
            if freq[idx] > 0 && c != i {
                freq[idx] -= 1;
                let max_freq = *freq.iter().max().unwrap();
                let new_remaining = remaining - 1;
                if max_freq <= (new_remaining + 1) / 2 {
                    sol.push(i);
                    c = i;
                    found = true;
                    break;
                } else {
                    freq[idx] += 1;
                }
            }
        }
        if !found {
            println!("{}", -1);
            return;
        }
    }
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
