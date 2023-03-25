// https://atcoder.jp/contests/abc014/tasks/abc014_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        mut X: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for i in 0..n {
        if X & 1 == 1 {
            ans += a[i];
        }
        X >>= 1;
    }
    println!("{}", ans);
}