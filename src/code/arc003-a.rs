// https://atcoder.jp/contests/arc003/tasks/arc003_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        r: String,
    }
    let mut gpa = 0.;
    for v in r.chars() {
        gpa += match v {
            'A' => 4.,
            'B' => 3.,
            'C' => 2.,
            'D' => 1.,
            _ => 0.,
        };
    }
    println!("{}", gpa / (N as f64));
}