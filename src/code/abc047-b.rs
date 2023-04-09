// https://atcoder.jp/contests/abc047/tasks/abc047_b

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        W: isize,
        H: isize,
        N: usize,
    }
    let mut x_mi = 0;
    let mut y_mi = 0;
    let mut x_ma = W;
    let mut y_ma = H;
    for _ in 0..N {
        input! {
            x: isize,
            y: isize,
            a: usize,
        }
        match a {
            1 => x_mi = max(x_mi, x),
            2 => x_ma = min(x_ma, x),
            3 => y_mi = max(y_mi, y),
            _ => y_ma = min(y_ma, y),
        }
    }
    println!("{}", max(0, x_ma - x_mi) * max(0, y_ma - y_mi));
}