// https://atcoder.jp/contests/arc137/tasks/arc137_a

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        R: usize,
    }
    let N = R - L;
    for d in 0..=N {
        for x in L..=R-N+d {
            let y = x + N - d;
            if gcd(x, y) == 1 {
                println!("{}", N - d);
                return;
            }
        }
    }
}