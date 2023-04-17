// https://atcoder.jp/contests/arc056/tasks/arc056_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        K: usize,
        L: usize,
    }
    let N = K / L;
    let M = K % L;
    let ans1 = B * N + A * M;
    let ans2 = B * (N + 1);
    println!("{}", min(ans1, ans2));
}