// https://atcoder.jp/contests/abc312/tasks/abc312_c

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: usize, N: usize, A: &Vec<usize>) -> usize {
    let mut ret = 0;
    for i in 0..N {
        if A[i] <= x {
            ret += 1;
        }
    }
    return ret;
}

#[allow(non_snake_case)]
fn g(x: usize, M: usize, B: &Vec<usize>) -> usize {
    let mut ret = 0;
    for i in 0..M {
        if x <= B[i] {
            ret += 1;
        }
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }
    let mut ok = 1_000_000_001;
    let mut ng = 0;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        if f(mid, N, &A) >= g(mid, M, &B) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}