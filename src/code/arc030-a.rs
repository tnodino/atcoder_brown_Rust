// https://atcoder.jp/contests/arc030/tasks/arc030_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    if n / 2 >= k {
        println!("YES");
    }
    else {
        println!("NO");
    }
}