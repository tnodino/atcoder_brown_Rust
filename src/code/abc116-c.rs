// https://atcoder.jp/contests/abc116/tasks/abc116_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        h: [usize; N],
    }
    let mut ans = h[0];
    for i in 1..N {
        if h[i-1] < h[i] {
            ans += h[i] - h[i-1];
        }
    }
    println!("{}", ans);
}