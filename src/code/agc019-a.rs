// https://atcoder.jp/contests/agc019/tasks/agc019_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
        H: usize,
        S: usize,
        D: usize,
        N: usize,
    }
    let S = min(Q * 4, min(H * 2, S));
    println!("{}", min(N * S, (N / 2 * D) + (N % 2 * S)));
}