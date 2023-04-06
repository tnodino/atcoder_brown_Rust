// https://atcoder.jp/contests/arc059/tasks/arc059_a

use proconio::input;
use proconio::fastout;
use std::isize::MAX;
use std::cmp::min;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [isize; N],
    }
    let mut ans = MAX;
    for x in -100..=100 {
        let mut res = 0;
        for i in 0..N {
            res += pow(a[i] - x, 2);
        }
        ans = min(ans, res);
    }
    println!("{}", ans);
}