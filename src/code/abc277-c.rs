// https://atcoder.jp/contests/abc277/tasks/abc277_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = HashMap::new();
    G.insert(1, Vec::new());
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        G.entry(A).or_insert(Vec::new()).push(B);
        G.entry(B).or_insert(Vec::new()).push(A);
    }
    let mut set = HashSet::new();
    let mut que = VecDeque::new();
    que.push_front(1usize);
    let mut ans: usize = 1;
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        let g = G.get(&pos).unwrap();
        for nxt in g {
            if set.contains(nxt) {
                continue;
            }
            set.insert(*nxt);
            que.push_back(*nxt);
            ans = max(ans, *nxt);
        }
    }
    println!("{}", ans);
}