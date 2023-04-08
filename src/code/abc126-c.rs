// https://atcoder.jp/contests/abc126/tasks/abc126_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let M = 1. / (N as f64);
    let mut ans = 0.;
    for i in 1..=N {
        let mut point = i;
        let mut x = M;
        while point < K {
            point *= 2;
            x *= 0.5;
        }
        ans += x;
    }
    println!("{}", ans);
}