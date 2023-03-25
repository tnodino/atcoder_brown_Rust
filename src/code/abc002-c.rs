// https://atcoder.jp/contests/abc002/tasks/abc002_3

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut xa: f64,
        mut ya: f64,
        mut xb: f64,
        mut yb: f64,
        xc: f64,
        yc: f64,
    }
    xa -= xc;
    ya -= yc;
    xb -= xc;
    yb -= yc;
    println!("{}", (xa * yb - xb * ya).abs() / 2.);
}