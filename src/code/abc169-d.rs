// https://atcoder.jp/contests/abc169/tasks/abc169_d

use proconio::input;
use proconio::fastout;
use libm::sqrt;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let M = sqrt(N as f64) as usize;
    let mut ans = 0;
    for i in 2..=M {
        for j in 1..100 {
            if N % pow(i, j) != 0 {
                break;
            }
            N /= pow(i, j);
            ans += 1;
        }
        while N % i == 0 {
            N /= i;
        }
    }
    if N > 1 {
        ans += 1;
    }
    println!("{}", ans);
}