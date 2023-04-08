// https://atcoder.jp/contests/abc046/tasks/abc046_b

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    println!("{}", K * pow(K - 1, N - 1));
}