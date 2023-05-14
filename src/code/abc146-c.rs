// https://atcoder.jp/contests/abc146/tasks/abc146_c

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(mut N: usize, A: usize, B: usize,) -> usize {
    if N == 0 {
        return 0;
    }
    let mut ret = A * N;
    while N > 0 {
        ret += B;
        N /= 10;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        X: usize,
    }
    let mut ok = 0;
    let mut ng = 1_000_000_001;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(mid, A, B) <= X {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}