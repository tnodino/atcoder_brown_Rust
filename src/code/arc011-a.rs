// https://atcoder.jp/contests/arc011/tasks/arc011_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        m: usize,
        n: usize,
        mut N: usize,
    }
    let mut ans = N;
    while N >= m {
        ans += N / m * n;
        N = N % m + N / m * n;
    }
    println!("{}", ans);
}