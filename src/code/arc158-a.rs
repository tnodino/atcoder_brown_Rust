// https://atcoder.jp/contests/arc158/tasks/arc158_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            x1: isize,
            x2: isize,
            x3: isize,
        }
        let a = (x1 + x2 + x3) / 3;
        if (x1 + x2 + x3) % 3 != 0 || x1 % 2 != a % 2 || x2 % 2 != a % 2 || x3 % 2 != a % 2 {
            println!("-1");
            continue;
        }
        let d = (x1 - a).abs() + (x2 - a).abs() + (x3 - a).abs();
        println!("{}", d / 4);
    }
}