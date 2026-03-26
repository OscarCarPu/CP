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

    let mut contains = vec![0u8; n];
    let mut contained = vec![0u8; n];

    let mut max_r = i64::MIN;
    for &(_, r, idx) in &ranges {
        if max_r >= r {
            contained[idx] = 1;
        }
        max_r = max_r.max(r);
    }

    let mut min_r = i64::MAX;
    for &(_, r, idx) in ranges.iter().rev() {
        if min_r <= r {
            contains[idx] = 1;
        }
        min_r = min_r.min(r);
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
