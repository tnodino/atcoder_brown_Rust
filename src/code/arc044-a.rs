// https://atcoder.jp/contests/arc044/tasks/arc044_a

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N == 1 {
        println!("Not Prime");
        return;
    }
    let M = sqrt(N as f64) as usize;
    for i in 2..=M {
        if N % i == 0 {
            if N % 2 == 0 || N % 3 == 0 || N % 5 == 0 {
                println!("Not Prime");
                return;
            }
            break;
        }
    }
    println!("Prime");
}