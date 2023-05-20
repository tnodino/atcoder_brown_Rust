// https://atcoder.jp/contests/arc107/tasks/arc107_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[allow(non_snake_case)]
fn f(N: isize, K: isize) -> usize {
    let N = N as usize;
    let K = K as usize;
    return min(K - 1, N * 2 + 1 - K);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        K: isize,
    }
    let mut ans = 0;
    for ab in 2..=N*2 {
        let cd = ab - K;
        if cd < 2 || N * 2 < cd {
            continue;
        }
        ans += f(N, ab) * f(N, cd);
    }
    println!("{}", ans);
}