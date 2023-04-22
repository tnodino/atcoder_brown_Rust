// https://atcoder.jp/contests/abc131/tasks/abc131_c

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[allow(non_snake_case)]
fn f(x: usize, c: usize, d: usize, l: usize) -> usize {
    return x - (x / c) - (x / d) + (x / l);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }
    let l = lcm(C, D);
    println!("{}", f(B, C, D, l) - f(A - 1, C, D, l));
}