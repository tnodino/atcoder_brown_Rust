// https://atcoder.jp/contests/abc182/tasks/abc182_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut op = vec![0; N+1];
    let mut ma = vec![0; N+1];
    for i in 0..N {
        op[i+1] = op[i] + A[i];
        ma[i+1] = max(ma[i], op[i+1]);
    }
    let mut pos = 0;
    let mut ans = 0;
    for i in 1..=N {
        ans = max(ans, pos + ma[i]);
        pos += op[i];
    }
    println!("{}", ans);
}