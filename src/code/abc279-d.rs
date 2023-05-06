// https://atcoder.jp/contests/abc279/tasks/abc279_d

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[allow(non_snake_case)]
fn f(A: usize, B: usize, x: usize) -> f64 {
    let A = A as f64;
    let B = B as f64;
    let x = x as f64;
    return (B * (x - 1.)) + (A / sqrt(x));
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let mut l = 1;
    let mut r = A;
    for _ in 0..200 {
        let m1 = (2 * l + r) / 3;
        let m2 = (l + 2 * r) / 3;
        if f(A, B, m1) < f(A, B, m2) {
            r = m2;
        }
        else {
            l = m1;
        }
    }
    let mut ans = A as f64;
    for i in l..=r {
        if f(A, B, i) < ans {
            ans = f(A, B, i);
        }
    }
    println!("{}", ans);
}