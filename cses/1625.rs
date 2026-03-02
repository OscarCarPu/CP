#![allow(unused_imports)]
use std::io::{self, BufRead};
use std::str::FromStr;

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
        self.tokens.pop().unwrap().parse().ok().unwrap()
    }
}

const N: usize = 7;

fn idx(r: i32, c: i32) -> usize {
    r as usize * N + c as usize
}

fn in_bounds(r: i32, c: i32) -> bool {
    r >= 0 && r < 7 && c >= 0 && c < 7
}

fn can_go(visited: &[bool], r: i32, c: i32) -> bool {
    in_bounds(r, c) && !visited[idx(r, c)]
}

fn backtrack(step: usize, r: i32, c: i32, visited: &mut [bool; 49], s: &[u8]) -> u64 {
    if r == 6 && c == 0 {
        return if step == 48 { 1 } else { 0 };
    }
    if step == 48 {
        return 0;
    }

    let up = can_go(visited, r - 1, c);
    let down = can_go(visited, r + 1, c);
    let left = can_go(visited, r, c - 1);
    let right = can_go(visited, r, c + 1);
    if !up && !down && left && right {
        return 0;
    }
    if up && down && !left && !right {
        return 0;
    }
    if !up && !down && !left && !right {
        return 0;
    }

    let dirs: [(u8, i32, i32); 4] = [(b'D', 1, 0), (b'R', 0, 1), (b'U', -1, 0), (b'L', 0, -1)];

    let ch = s[step];
    let mut ans = 0u64;

    for &(d, dr, dc) in &dirs {
        if ch != b'?' && ch != d {
            continue;
        }
        let nr = r + dr;
        let nc = c + dc;
        if !in_bounds(nr, nc) || visited[idx(nr, nc)] {
            continue;
        }
        visited[idx(nr, nc)] = true;
        ans += backtrack(step + 1, nr, nc, visited, s);
        visited[idx(nr, nc)] = false;
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    let s: String = scanner.next();
    let mut visited = [false; 49];
    visited[0] = true;
    println!("{}", backtrack(0, 0, 0, &mut visited, s.as_bytes()));
}
