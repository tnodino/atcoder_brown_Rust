// https://atcoder.jp/contests/abc189/tasks/abc189_c

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    for l in 0..N {
        let mut mi = 1<<32;
        for r in l..N {
            mi = min(mi, A[r]);
            ans = max(ans, mi * (r - l + 1));
        }
    }
    println!("{}", ans);
}