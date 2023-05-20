// https://atcoder.jp/contests/arc145/tasks/arc145_b

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

#[allow(non_snake_case)]
fn f(X: isize, A: isize, B: isize) -> isize {
    return X / A * min(A, B) + min(X % A, B - 1);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        A: isize,
        B: isize,
    }
    println!("{}", max(f(N, A, B) - f(A - 1, A, B), 0));
}