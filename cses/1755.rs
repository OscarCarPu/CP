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
    let mut freq: HashMap<char, i64> = HashMap::new();
    for c in s.chars() {
        freq.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut char_impar: Option<char> = None;
    for (c, f) in freq.iter() {
        if f % 2 == 1 {
            if char_impar != None {
                println!("NO SOLUTION");
                return;
            } else {
                char_impar = Some(*c);
            }
        }
    }
    let mut first_half = String::new();
    for (c, f) in freq.iter() {
        if Some(*c) == char_impar {
            continue;
        }
        for _ in 1..=f / 2 {
            first_half.push(*c);
        }
    }
    let mut middle = String::new();
    if let Some(c) = char_impar {
        for _ in 0..*freq.get(&c).unwrap() {
            middle.push(c);
        }
    }
    println!(
        "{}",
        first_half.clone() + &middle + &first_half.chars().rev().collect::<String>()
    );
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
