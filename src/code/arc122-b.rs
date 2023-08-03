// https://atcoder.jp/contests/arc122/tasks/arc122_b

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: f64, N: usize, A: &Vec<f64>) -> f64 {
    let M = N as f64;
    let mut res = M * x;
    for i in 0..N {
        if x * 2. < A[i] {
            res += A[i] - (x * 2.);
        }
    }
    res /= M;
    return res;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [f64; N],
    }
    let mut l = 0.;
    let mut r = 1_000_000_000.;
    for _ in 0..100 {
        let m1 = (l + l + r) / 3.;
        let m2 = (l + r + r) / 3.;
        if f(m1, N, &A) > f(m2, N, &A) {
            l = m1;
        }
        else {
            r = m2;
        }
    }
    println!("{}", f(l, N, &A));
}