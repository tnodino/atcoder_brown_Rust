// https://atcoder.jp/contests/abc198/tasks/abc198_c

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: f64,
        X: f64,
        Y: f64,
    }
    let d = hypot(X, Y);
    if d == R {
        println!("1");
    }
    else if d <= R * 2. {
        println!("2");
    }
    else {
        println!("{}", (d / R + 0.999999) as usize);
    }
}