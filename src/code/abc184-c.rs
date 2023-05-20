// https://atcoder.jp/contests/abc184/tasks/abc184_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        r1: isize,
        c1: isize,
        r2: isize,
        c2: isize,
    }
    let r = r2 - r1;
    let c = c2 - c1;
    if r == 0 && c == 0 {
        println!("0");
    }
    else if r == c || r == -c {
        println!("1");
    }
    else if r.abs() + c.abs() <= 3 {
        println!("1");
    }
    else if (r + c) % 2 == 0 {
        println!("2");
    }
    else if r.abs() + c.abs() <= 6 {
        println!("2");
    }
    else if (r + c).abs() <= 3 || (r - c).abs() <= 3 {
        println!("2");
    }
    else {
        println!("3");
    }
}