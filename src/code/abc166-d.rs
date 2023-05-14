// https://atcoder.jp/contests/abc166/tasks/abc166_d

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
    }
    for A in -120..=120 {
        for B in -120..=120 {
            if pow(A, 5) - pow(B, 5) == X {
                println!("{} {}", A, B);
                return;
            }
        }
    }
}