// https://atcoder.jp/contests/abc090/tasks/arc091_a

use proconio::input;
use proconio::fastout;
use std::mem::swap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
        mut M: usize,
    }
    if N > M {
        swap(&mut N, &mut M);
    }
    if N == 1 && M == 1 {
        println!("1");
    }
    else if N == 1 && M > 1 {
        println!("{}", M - 2);
    }
    else {
        println!("{}", (N - 2) * (M - 2));
    }
}