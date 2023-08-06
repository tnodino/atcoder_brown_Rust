// https://atcoder.jp/contests/abc227/tasks/abc227_c

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let A = sqrt(N as f64) as usize;
    let mut ans: usize = 0;
    for a in 1..=A {
        let M = N / a;
        let B = sqrt(M as f64) as usize;
        for b in a..=B {
            ans += M / b + 1 - b;
        }
    }
    println!("{}", ans);
}