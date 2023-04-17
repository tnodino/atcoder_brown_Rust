// https://atcoder.jp/contests/abc060/tasks/arc073_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: usize,
        t: [usize; N],
    }
    let mut rev = 0;
    let mut ans = 0;
    for i in 1..N {
        if rev + T <= t[i] {
            ans += T;
        }
        else {
            ans += t[i] - rev;
        }
        rev = t[i]
    }
    println!("{}", ans + T);
}