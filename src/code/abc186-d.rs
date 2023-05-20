// https://atcoder.jp/contests/abc186/tasks/abc186_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
    }
    A.sort();
    let mut s = A.iter().sum::<isize>();
    let mut ans = 0;
    for i in 0..N {
        s -= A[i];
        ans += s - A[i] * (N as isize - i as isize - 1);
    }
    println!("{}", ans);
}