// https://atcoder.jp/contests/abc180/tasks/abc180_d

use proconio::input;
use proconio::fastout;
use num::pow;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        A: usize,
        B: usize,
    }
    let mut ans = 0;
    for a in 0..100 {
        if X >= Y / pow(A, a) {
            break;
        }
        let x = Y - X * pow(A, a);
        let res = a + (x - 1) / B;
        ans = max(ans, res);
    }
    println!("{}", ans);
}