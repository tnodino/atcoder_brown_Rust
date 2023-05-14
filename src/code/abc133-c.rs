// https://atcoder.jp/contests/abc133/tasks/abc133_c

use proconio::input;
use std::cmp::min;

const MOD: usize = 2019;

#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        R: usize,
    }
    let mut ans = 2019;
    'outer: for i in L..=R {
        for j in i+1..=R {
            ans = min(ans, i * j % MOD);
            if ans == 0 {
                break 'outer;
            }
        }
    }
    println!("{}", ans);
}