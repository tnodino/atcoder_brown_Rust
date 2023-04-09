// https://atcoder.jp/contests/abc060/tasks/abc060_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    for i in 1..=100 {
        if (A * i) % B == C {
            println!("YES");
            return;
        }
    }
    println!("NO");
}