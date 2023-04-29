// https://atcoder.jp/contests/abc053/tasks/arc068_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: usize,
    }
    let n = x / 11;
    let m = x % 11;
    let k;
    if m == 0 {
        k = 0;
    }
    else if m <= 6 {
        k = 1;
    }
    else {
        k = 2;
    }
    println!("{}", n * 2 + k);
}