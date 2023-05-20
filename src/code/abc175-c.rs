// https://atcoder.jp/contests/abc175/tasks/abc175_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: isize,
        mut K: isize,
        D: isize,
    }
    X = max(X, -X);
    let cnt = X / D;
    if cnt >= K {
        println!("{}", X - K * D);
        return;
    }
    X -= cnt * D;
    K -= cnt;
    if K % 2 == 0 {
        println!("{}", X);
    }
    else {
        println!("{}", (X - D).abs());
    }
}