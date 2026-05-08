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
#[derive(Clone)]
struct MexSet {
    present: HashSet<usize>,
    absent: BTreeSet<usize>,
    max_val: usize,
}
#[allow(unused)]
impl MexSet {
    fn new(max_val: usize) -> Self {
        Self {
            present: HashSet::new(),
            absent: (0..=max_val).collect(),
            max_val,
        }
    }

    fn insert(&mut self, val: usize) {
        if self.present.insert(val) {
            self.absent.remove(&val);
        }
    }
    fn merge(&mut self, other: &Self) {
        self.present.extend(other.present.iter());
        self.absent.retain(|&x| !other.present.contains(&x));
    }
    fn remove(&mut self, val: usize) {
        if self.present.remove(&val) {
            self.absent.insert(val);
        }
    }
    fn mex(&self) -> usize {
        *self.absent.iter().next().unwrap()
    }
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
    let v: Vec<usize> = (0..n * 2).map(|_| sc.next()).collect();
    let two_n = 2 * n;

    let mut first = vec![usize::MAX; n];
    let mut second = vec![0usize; n];
    for (idx, &val) in v.iter().enumerate() {
        if first[val] == usize::MAX {
            first[val] = idx;
        } else {
            second[val] = idx;
        }
    }

    let mut valid = vec![false; n];
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| second[i] - first[i]);
    for &i in &order {
        if second[i] - first[i] <= 2 {
            valid[i] = true;
        } else {
            let inner = v[first[i] + 1];
            if inner == v[second[i] - 1] && valid[inner] {
                valid[i] = true;
            }
        }
    }

    let mut parent = vec![usize::MAX; n];
    for k in 0..n {
        if !valid[k] || first[k] == 0 || second[k] + 1 >= two_n {
            continue;
        }
        let p = v[first[k] - 1];
        if v[second[k] + 1] == p
            && first[p] == first[k] - 1
            && second[p] == second[k] + 1
            && valid[p]
        {
            parent[k] = p;
        }
    }

    let mut visited = vec![false; n];
    let mut present = vec![false; n + 1];
    let mut ans = 1;
    let mut stack: Vec<usize> = Vec::new();

    for leaf in 0..n {
        if !valid[leaf] || visited[leaf] || second[leaf] - first[leaf] >= 3 {
            continue;
        }

        stack.clear();
        let mut mex = 0;

        let mut cur = leaf;
        present[cur] = true;
        stack.push(cur);
        if second[cur] - first[cur] == 2 {
            let m = v[first[cur] + 1];
            if !present[m] {
                present[m] = true;
                stack.push(m);
            }
        }
        visited[cur] = true;
        while mex <= n && present[mex] {
            mex += 1;
        }
        ans = max(ans, mex);

        while parent[cur] != usize::MAX {
            cur = parent[cur];
            visited[cur] = true;
            if !present[cur] {
                present[cur] = true;
                stack.push(cur);
            }
            while mex <= n && present[mex] {
                mex += 1;
            }
            ans = max(ans, mex);
        }

        for &u in &stack {
            present[u] = false;
        }
    }

    println!("{}", ans);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    let t: usize = scanner.next();
    for _ in 0..t {
        solve(&mut scanner);
    }
}
