// https://atcoder.jp/contests/abc195/tasks/abc195_b

use proconio::input;
use proconio::fastout;
use std::usize::MAX;
use std::cmp::{min, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        mut W: usize,
    }
    W *= 1000;
    let mut mi = MAX;
    let mut ma = 0;
    for i in 1..=1_000_000 {
        if A * i <= W && W <= B * i {
            mi = min(mi, i);
            ma = max(ma, i);
        }
    }
    if ma == 0 {
        println!("UNSATISFIABLE")
    }
    else {
        println!("{} {}", mi, ma);
    }
}