// https://atcoder.jp/contests/abc267/tasks/abc267_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [isize; N],
    }
    let mut res = 0;
    let mut s = 0;
    for i in 0..M {
        res += A[i] * (i as isize + 1);
        s += A[i];
    }
    let mut ans = res;
    for i in M..N {
        res -= s;
        res += A[i] * (M as isize);
        s -= A[i-M];
        s += A[i];
        ans = max(ans, res);
    }
    println!("{}", ans);
}