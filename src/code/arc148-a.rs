// https://atcoder.jp/contests/arc148/tasks/arc148_a

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut g = 0;
    for i in 0..N-1 {
        g = gcd(g, (A[i] - A[i+1]).abs());
    }
    println!("{}", match g {
        1 => 2,
        _ => 1,
    });
}