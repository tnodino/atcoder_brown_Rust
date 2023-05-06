// https://atcoder.jp/contests/abc067/tasks/arc078_a

use proconio::input;
use proconio::fastout;
use std::isize::MAX;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [isize; N],
    }
    let s = a.iter().sum::<isize>();
    let mut x = 0;
    let mut ans = MAX;
    for i in 0..N-1 {
        x += a[i];
        let y = s - x;
        ans = min(ans, (x - y).abs());
    }
    println!("{}", ans);
}