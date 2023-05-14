// https://atcoder.jp/contests/abc144/tasks/abc144_d

use proconio::input;
use proconio::fastout;
use std::f64::consts::PI;
use libm::tan;

#[allow(non_snake_case)]
fn f(a: f64, b: f64, theta: f64) -> f64 {
    if a * tan(theta) <= b {
        return a * a * b - a * a * a * tan(theta) / 2.;
    }
    else {
        return b * b / tan(theta) * a / 2.;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }
    let mut ok = 0.;
    let mut ng = 90. * (PI / 180.);
    for _ in 0..100 {
        let mid = (ok + ng) / 2.;
        if f(a, b, mid) >= x {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok / (PI / 180.));
}