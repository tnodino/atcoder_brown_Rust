// https://atcoder.jp/contests/agc008/tasks/agc008_a

use proconio::input;
use proconio::fastout;
use std::usize::MAX;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: isize,
        y: isize,
    }
    let s = [x,  x, -x, -x];
    let t = [y, -y,  y, -y];
    let b = [0,  1,  1,  2];
    let mut ans = MAX;
    for i in 0..4 {
        if s[i] <= t[i] {
            ans = min(ans, (t[i] - s[i]) as usize + b[i]);
        }
    }
    println!("{}", ans);
}