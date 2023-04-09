// https://atcoder.jp/contests/abc040/tasks/abc040_b

use proconio::input;
use proconio::fastout;
use std::usize::MAX;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = MAX;
    for i in 1..=1000 {
        for j in i..=1000 {
            if i * j > n {
                break;
            }
            ans = min(ans, (n - i * j) + (j - i));
        }
    }
    println!("{}", ans);
}