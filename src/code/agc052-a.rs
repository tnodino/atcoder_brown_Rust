// https://atcoder.jp/contests/agc052/tasks/agc052_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            N: usize,
            _S1: String,
            _S2: String,
            _S3: String,
        }
        println!("{}{}0", "0".repeat(N), "1".repeat(N));
    }
}