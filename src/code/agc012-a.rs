// https://atcoder.jp/contests/agc012/tasks/agc012_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; 3*N],
    }
    a.sort();
    let mut ans = 0;
    for i in (N..3*N).step_by(2) {
        ans += a[i];
    }
    println!("{}", ans);
}