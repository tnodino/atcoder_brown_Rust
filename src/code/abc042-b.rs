// https://atcoder.jp/contests/abc042/tasks/abc042_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        _L: usize,
        mut S: [String; N],
    }
    S.sort();
    println!("{}", S.join(""));
}