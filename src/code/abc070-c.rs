// https://atcoder.jp/contests/abc070/tasks/abc070_c

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: [usize; N],
    }
    let mut ans = 1;
    for i in 0..N {
        ans = lcm(ans, T[i]);
    }
    println!("{}", ans);
}