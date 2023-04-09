// https://atcoder.jp/contests/abc055/tasks/arc069_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut M: usize,
    }
    let cnt = min(N, M / 2);
    M -= cnt * 2;
    println!("{}", cnt + M / 4);
}