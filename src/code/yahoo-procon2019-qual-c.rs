// https://atcoder.jp/contests/yahoo-procon2019-qual/tasks/yahoo_procon2019_qual_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: isize,
        A: isize,
        B: isize,
    }
    if B - A <= 2 {
        println!("{}", K + 1);
    }
    else {
        if K < A {
            println!("{}", K + 1);
        }
        else {
            let N = K - A + 1;
            println!("{}", A + (N / 2) * (B - A) + (N % 2));
        }
    }
}