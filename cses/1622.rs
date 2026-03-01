#![allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::str::FromStr;

#[allow(unused)]
const MOD: i64 = 1_000_000_007;

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
fn pow_mod(mut base: i64, mut exp: i64) -> i64 {
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

fn solve(sc: &mut Scanner<io::StdinLock>) {
    let s: String = sc.next();
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();

    let mut sol = Vec::new();
    fn build_sol(
        chars: &Vec<char>,
        used: &mut Vec<bool>,
        current: &mut String,
        sol: &mut Vec<String>,
        n: usize,
    ) {
        if current.len() == n {
            sol.push(current.clone());
            return;
        }
        for i in 0..n {
            if used[i] {
                continue;
            }
            if i > 0 && chars[i] == chars[i - 1] && !used[i - 1] {
                continue;
            }

            used[i] = true;
            current.push(chars[i]);
            build_sol(chars, used, current, sol, n);
            current.pop();
            used[i] = false;
        }
    }
    build_sol(
        &chars,
        &mut vec![false; chars.len()],
        &mut String::new(),
        &mut sol,
        s.len(),
    );
    println!("{}", sol.len());
    for s in sol {
        println!("{}", s);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
