// https://atcoder.jp/contests/arc129/tasks/arc129_b

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut L = 0;
    let mut R = 1_000_000_000;
    for _ in 0..N {
        input! {
            l: usize,
            r: usize,
        }
        L = max(L, l);
        R = min(R, r);
        if L <= R {
            println!("0");
        }
        else {
            println!("{}", (L - R + 1) / 2);
        }
    }
}