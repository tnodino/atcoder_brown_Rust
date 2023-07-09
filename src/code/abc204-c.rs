// https://atcoder.jp/contests/abc204/tasks/abc204_c

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

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
    }
    let mut ans = 0;
    for i in 0..N {
        let mut flg = vec![false; N];
        flg[i] = true;
        let mut que = VecDeque::new();
        que.push_back(i);
        while !que.is_empty() {
            let pos = que.pop_front().unwrap();
            for nxt in G[pos].iter() {
                if flg[*nxt] {
                    continue;
                }
                flg[*nxt] = true;
                que.push_back(*nxt);
            }
        }
        ans += flg.iter().filter(|&x| *x).count();
    }
    println!("{}", ans);
}