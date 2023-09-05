// https://atcoder.jp/contests/abc131/tasks/abc131_c

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

fn f(x: usize, c: usize, d: usize) -> usize {
    let lcm = lcm(c, d);
    return x - (x / c + x / d - x / lcm);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C, D): (usize, usize, usize, usize),
    }
    println!("{}", f(B, C, D) - f(A-1, C, D));
}