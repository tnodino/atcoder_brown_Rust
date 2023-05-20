// https://atcoder.jp/contests/arc126/tasks/arc126_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            mut N2: usize,
            mut N3: usize,
            mut N4: usize,
        }
        let mut ans = 0;
        let cnt = min(N3 / 2, N4);
        ans += cnt;
        N3 -= cnt * 2;
        N4 -= cnt;
        let cnt = min(N2 / 2, N3 / 2);
        ans += cnt;
        N2 -= cnt * 2;
        let cnt = min(N2, N4 / 2);
        ans += cnt;
        N2 -= cnt;
        N4 -= cnt * 2;
        let cnt = min(N2 / 3, N4);
        ans += cnt;
        N2 -= cnt * 3;
        let cnt = N2 / 5;
        ans += cnt;
        println!("{}", ans);
    }
}