// https://atcoder.jp/contests/abc255/tasks/abc255_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
        mut A: isize,
        mut D: isize,
        N: isize,
    }
    if D < 0 {
        A = A + D * (N - 1);
        D = -D;
    }
    let mut l = 0;
    let mut r = N - 1;
    for _ in 0..100 {
        let mid = (l + r) / 2;
        if A + D * mid < X {
            l = mid;
        }
        else {
            r = mid;
        }
    }
    println!("{}", min((X - (A + D * l)).abs(), (X - (A + D * r)).abs()));
}