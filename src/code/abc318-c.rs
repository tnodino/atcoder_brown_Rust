// https://atcoder.jp/contests/abc318/tasks/abc318_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize,
        P: usize,
        mut F: [usize; N],
    }
    F.sort_by(|a, b| b.cmp(&a));
    let mut s = F.iter().sum::<usize>();
    let mut ans = s;
    for i in 0..N {
        if i % D == 0 {
            s += P;
        }
        s -= F[i];
        ans = min(ans, s);
    }
    println!("{}", ans);
}