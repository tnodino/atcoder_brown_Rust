// https://atcoder.jp/contests/arc013/tasks/arc013_2

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        C: usize,
    }
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for _ in 0..C {
        input! {
            N: usize,
            M: usize,
            L: usize,
        }
        let mut arr = [N, M, L];
        arr.sort();
        x = max(x, arr[0]);
        y = max(y, arr[1]);
        z = max(z, arr[2]);
    }
    println!("{}", x * y * z);
}