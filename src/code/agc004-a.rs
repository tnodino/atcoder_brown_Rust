// https://atcoder.jp/contests/agc004/tasks/agc004_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let mut arr = [A, B, C];
    arr.sort();
    for i in 0..3 {
        if arr[i] % 2 == 0 {
            println!("0");
            return;
        }
    }
    println!("{}", arr[0] * arr[1]);
}