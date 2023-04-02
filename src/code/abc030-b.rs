// https://atcoder.jp/contests/abc030/tasks/abc030_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut n: f64,
        m: f64,
    }
    n %= 12.;
    let a = n * 30. + m * 0.5;
    let b = m * 6.;
    let mut x = (a - b).abs();
    if x > 180. {
        x = 360. - x;
    }
    println!("{}", x);
}