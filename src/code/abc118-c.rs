// https://atcoder.jp/contests/abc118/tasks/abc118_c

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        ans = gcd(ans, A[i]);
    }
    println!("{}", ans);
}