// https://atcoder.jp/contests/abc169/tasks/abc169_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: f64,
    }
    let B = (B * 100. + 0.5) as usize;
    println!("{}", A * B / 100);
}