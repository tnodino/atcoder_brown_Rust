// https://atcoder.jp/contests/abc109/tasks/abc109_c

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: isize,
        x: [isize; N],
    }
    let mut ans = (X - x[0]).abs();
    for i in 0..N {
        ans = gcd(ans, (X - x[i]).abs());
    }
    println!("{}", ans);
}