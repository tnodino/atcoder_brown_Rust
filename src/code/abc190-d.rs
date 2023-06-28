// https://atcoder.jp/contests/abc190/tasks/abc190_d

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    while N % 2 == 0 {
        N /= 2;
    }
    let mut ans = 0;
    let M = sqrt(N as f64) as usize;
    for i in 1..=M {
        if N % i == 0 {
            ans += 1;
            if N / i != i {
                ans += 1;
            }
        }
    }
    println!("{}", ans * 2);
}