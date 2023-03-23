// https://atcoder.jp/contests/abc284/tasks/abc284_d

use proconio::input;
use proconio::fastout;
use libm::pow;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            N: usize,
        }
        let M = pow(N as f64, 1. / 3.) as usize;
        for i in 2..=M {
            if N % i != 0 {
                continue;
            }
            if (N / i) % i == 0 {
                println!("{} {}", i, N / i / i);
            }
            else {
                println!("{} {}", sqrt((N / i) as f64), i);
            }
            break;
        }
    }
}