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
    let n: usize = sc.next();

    let mut board: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; n];
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, 0));
    while let Some((i, j, d)) = queue.pop_front() {
        if board[i][j] != usize::MAX {
            continue;
        }
        board[i][j] = d;
        if i + 2 < n && j + 1 < n && board[i + 2][j + 1] == usize::MAX {
            queue.push_back((i + 2, j + 1, d + 1));
        }
        if i + 2 < n && j >= 1 && board[i + 2][j - 1] == usize::MAX {
            queue.push_back((i + 2, j - 1, d + 1));
        }
        if i >= 2 && j + 1 < n && board[i - 2][j + 1] == usize::MAX {
            queue.push_back((i - 2, j + 1, d + 1));
        }
        if i >= 2 && j >= 1 && board[i - 2][j - 1] == usize::MAX {
            queue.push_back((i - 2, j - 1, d + 1));
        }
        if i + 1 < n && j + 2 < n && board[i + 1][j + 2] == usize::MAX {
            queue.push_back((i + 1, j + 2, d + 1));
        }
        if i + 1 < n && j >= 2 && board[i + 1][j - 2] == usize::MAX {
            queue.push_back((i + 1, j - 2, d + 1));
        }
        if i >= 1 && j + 2 < n && board[i - 1][j + 2] == usize::MAX {
            queue.push_back((i - 1, j + 2, d + 1));
        }
        if i >= 1 && j >= 2 && board[i - 1][j - 2] == usize::MAX {
            queue.push_back((i - 1, j - 2, d + 1));
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ", board[i][j]);
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
