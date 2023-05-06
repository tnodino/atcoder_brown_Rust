// https://atcoder.jp/contests/abc103/tasks/abc103_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        ans += a[i] - 1;
    }
    println!("{}", ans);
}