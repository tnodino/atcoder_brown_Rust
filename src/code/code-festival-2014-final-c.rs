// https://atcoder.jp/contests/code-festival-2014-final/tasks/code_festival_final_c

use proconio::input;
use proconio::fastout;

fn f(mut x: u128) -> u128 {
    let mut a = 1;
    let b = x;
    let mut ret = 0;
    while x > 0 {
        ret += x % 10 * a;
        x /= 10;
        a *= b;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: u128,
    }
    for i in 10..=10000 {
        if f(i) == A {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}