// https://atcoder.jp/contests/abc178/tasks/abc178_c

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret *= x;
            ret %= MOD;
        }
        x *= x;
        x %= MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = bin_power(10, N);
    ans += MOD * 10;
    ans -= bin_power(9, N);
    ans -= bin_power(9, N);
    ans += bin_power(8, N);
    ans %= MOD;
    println!("{}", ans);
}