// https://atcoder.jp/contests/abc166/tasks/abc166_d

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
    }
    let N = 200;
    for a in -N..N {
        for b in -N..N {
            if pow(a, 5) - pow(b, 5) == X {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}