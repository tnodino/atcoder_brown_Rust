// https://atcoder.jp/contests/abc211/tasks/abc211_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const INF: usize = 1<<60;
const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        G[A-1].push(B-1);
        G[B-1].push(A-1);
    }
    let mut dist = vec![INF; N];
    dist[0] = 0;
    let mut cnt = vec![0; N];
    cnt[0] = 1;
    let mut que = VecDeque::new();
    que.push_back(0);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if dist[pos] + 1 < dist[*nxt] {
                dist[*nxt] = dist[pos] + 1;
                cnt[*nxt] = cnt[pos];
            }
            else if dist[pos] + 1 == dist[*nxt] {
                cnt[*nxt] += cnt[pos];
                cnt[*nxt] %= MOD;
                continue;
            }
            else {
                continue;
            }
            que.push_back(*nxt);
        }
    }
    println!("{}", cnt[N-1]);
}