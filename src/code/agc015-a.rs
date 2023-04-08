// https://atcoder.jp/contests/agc015/tasks/agc015_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    if A > B {
        println!("0");
        return;
    }
    if A < B && N == 1 {
        println!("0");
        return;
    }
    println!("{}", (B * (N - 1) + A) - (A * (N - 1) + B) + 1);
}