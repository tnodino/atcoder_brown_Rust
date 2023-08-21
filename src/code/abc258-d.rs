// https://atcoder.jp/contests/abc258/tasks/abc258_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
    }
    let mut s = 0;
    let mut mi = 1<<60;
    let mut ans = 1<<60;
    for i in 1..=min(N, X) {
        input! {
            A: usize,
            B: usize,
        }
        s += A + B;
        mi = min(mi, B);
        ans = min(ans, s + (X - i) * mi);
    }
    println!("{}", ans);
}