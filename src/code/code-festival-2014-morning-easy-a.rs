// https://atcoder.jp/contests/code-festival-2014-morning-easy/tasks/code_festival_morning_easy_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }
    let mut su = 0.;
    for i in 0..n-1 {
        su += a[i+1] - a[i];
    }
    println!("{}", su / (n - 1) as f64);
}