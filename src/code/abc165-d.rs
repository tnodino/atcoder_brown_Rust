// https://atcoder.jp/contests/abc165/tasks/abc165_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[allow(non_snake_case)]
fn f(x: usize, A: usize, B: usize) -> usize {
    return (A * x / B) - (A * (x / B));
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        N: usize,
    }
    println!("{}", f(min(B - 1, N), A, B));
}