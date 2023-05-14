// https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_c

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    }
    if 0 < c - a - b && 4 * a * b < pow(c - a - b, 2) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}