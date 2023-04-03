// https://atcoder.jp/contests/arc057/tasks/arc057_a

use proconio::input;
use proconio::fastout;

const MAX: usize = 2_000_000_000_000;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        K: usize,
    }
    if K == 0 {
        println!("{}", MAX - A);
        return;
    }
    let mut ans = 0;
    while A < MAX {
        A += A * K + 1;
        ans += 1;
    }
    println!("{}", ans);
}