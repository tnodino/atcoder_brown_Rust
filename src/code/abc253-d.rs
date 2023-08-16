// https://atcoder.jp/contests/abc253/tasks/abc253_d

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

fn f(x: usize) -> usize {
    return (x + 1) * x / 2;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    let l = lcm(A, B);
    println!("{}", f(N) + (l * f(N/l)) - (A * f(N/A)) - (B * f(N/B)));
}