// https://atcoder.jp/contests/abc317/tasks/abc317_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[allow(non_snake_case)]
fn dfs(pos: usize, N: usize, G: &Vec<Vec<(usize, usize)>>, flg: &mut Vec<bool>) -> usize {
    let mut ret = 0;
    for (nxt, c) in G[pos].iter() {
        if !flg[*nxt] {
            flg[*nxt] = true;
            ret = max(ret, dfs(*nxt, N, &G, flg) + c);
            flg[*nxt] = false;
        }
    }
    return ret;
}

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
            C: usize,
        }
        G[A-1].push((B-1, C));
        G[B-1].push((A-1, C));
    }
    let mut ans = 0;
    for i in 0..N {
        let mut flg = vec![false; N];
        flg[i] = true;
        ans = max(ans, dfs(i, N, &G, &mut flg));
    }
    println!("{}", ans);
}