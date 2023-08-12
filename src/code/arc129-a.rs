// https://atcoder.jp/contests/arc129/tasks/arc129_a

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: usize,
        R: usize,
    }
    let mut ans = 0;
    for i in 0..60 {
        if N & (1 << i) > 0 {
            let l = max(L, pow(2, i));
            let r = min(R, pow(2, i+1)-1);
            if l <= r {
                ans += r - l + 1;
            }
        }
    }
    println!("{}", ans);
}