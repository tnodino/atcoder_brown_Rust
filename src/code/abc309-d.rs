// https://atcoder.jp/contests/abc309/tasks/abc309_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N1: usize,
        N2: usize,
        M: usize,
    }
    let mut G1 = vec![Vec::new(); N1];
    let mut G2 = vec![Vec::new(); N2];
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        if a <= N1 {
            G1[a-1].push(b-1);
            G1[b-1].push(a-1);
        }
        else {
            G2[a-1-N1].push(b-1-N1);
            G2[b-1-N1].push(a-1-N1);
        }
    }
    let mut dist1 = vec![0; N1];
    let mut flg = vec![false; N1];
    flg[0] = true;
    let mut que = VecDeque::new();
    que.push_back(0);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G1[pos].iter() {
            if flg[*nxt] {
                continue;
            }
            dist1[*nxt] = dist1[pos] + 1;
            flg[*nxt] = true;
            que.push_back(*nxt);
        }
    }
    let mut dist2 = vec![0; N2];
    let mut flg = vec![false; N2];
    flg[N2-1] = true;
    let mut que = VecDeque::new();
    que.push_back(N2-1);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G2[pos].iter() {
            if flg[*nxt] {
                continue;
            }
            dist2[*nxt] = dist2[pos] + 1;
            flg[*nxt] = true;
            que.push_back(*nxt);
        }
    }
    println!("{}", dist1.iter().max().unwrap() + dist2.iter().max().unwrap() + 1);
}