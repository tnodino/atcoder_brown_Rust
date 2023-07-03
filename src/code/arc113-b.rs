// https://atcoder.jp/contests/arc113/tasks/arc113_b

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn bin_power(mut x: usize, mut k: usize, MOD: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let mut k = bin_power(B, C, 4);
    if k == 0 {
        k = 4;
    }
    println!("{}", bin_power(A, k, 10));
}