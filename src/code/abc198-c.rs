// https://atcoder.jp/contests/abc198/tasks/abc198_c

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (R, X, Y): (f64, f64, f64),
    }
    let dist = hypot(X, Y);
    if dist == R {
        println!("1");
    }
    else if dist <= R * 2. {
        println!("2");
    }
    else {
        println!("{}", ((dist / R).ceil()));
    }
}