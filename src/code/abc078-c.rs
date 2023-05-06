// https://atcoder.jp/contests/abc078/tasks/arc085_a

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    println!("{}", (1900 * M + 100 * (N - M)) * pow(2, M));
}