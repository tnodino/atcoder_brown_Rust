// https://atcoder.jp/contests/abc101/tasks/arc099_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        _A: [usize; N],
    }
    println!("{}", (N + K - 3) / (K - 1));
}