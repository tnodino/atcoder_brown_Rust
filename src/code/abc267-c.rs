// https://atcoder.jp/contests/abc267/tasks/abc267_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [isize; N],
    }
    let mut res = 0;
    let mut sum = 0;
    for i in 0..M {
        res += A[i] * (i + 1) as isize;
        sum += A[i];
    }
    let mut ans = res;
    for i in M..N {
        res -= sum;
        sum -= A[i-M];
        res += A[i] * M as isize;
        sum += A[i];
        ans = max(ans, res);
    }
    println!("{}", ans);
}