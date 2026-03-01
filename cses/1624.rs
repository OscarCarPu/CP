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

fn sub_solve(row: usize, board: &Vec<Vec<char>>, queens: &mut Vec<usize>, sol: &mut i32) {
    if row == 8 {
        *sol += 1;
        return;
    }
    'col: for col in 0..8 {
        if board[row][col] == '*' {
            continue;
        }
        for r in 0..row {
            let c = queens[r];
            if c == col || (row as i32 - r as i32).abs() == (col as i32 - c as i32).abs() {
                continue 'col;
            }
        }
        queens.push(col);
        sub_solve(row + 1, board, queens, sol);
        queens.pop();
    }
}

fn solve(sc: &mut Scanner<io::StdinLock>) {
    let mut board = Vec::new();
    for _ in 0..8 {
        let line: String = sc.next();
        board.push(line.chars().collect::<Vec<char>>());
    }
    let mut sol = 0;
    let mut queens = Vec::new();
    sub_solve(0, &board, &mut queens, &mut sol);
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    solve(&mut scanner);
}
