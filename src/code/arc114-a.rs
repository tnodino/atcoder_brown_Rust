// https://atcoder.jp/contests/arc114/tasks/arc114_a

use proconio::input;
use num::integer::gcd;
use std::cmp::min;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: [usize; N],
    }
    let mut P = Vec::new();
    'outer: for i in 2..=50 {
        for j in 2..i {
            if i % j == 0 {
                continue 'outer;
            }
        }
        P.push(i);
    }
    let M = P.len();
    let mut ans = 1<<60;
    'outer: for bit in 1..1<<M {
        let mut res = 1;
        for i in 0..M {
            if bit & (1 << i) > 0 {
                res *= P[i];
            }
        }
        for i in 0..N {
            if gcd(res, X[i]) == 1 {
                continue 'outer;
            }
        }
        ans = min(ans, res);
    }
    println!("{}", ans);
}