// https://atcoder.jp/contests/abc123/tasks/abc123_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        C: usize,
        D: usize,
        E: usize,
    }
    let arr = [A, B, C, D, E];
    let mi = *arr.iter().min().unwrap();
    println!("{}", (N + mi - 1) / mi + 4);
}