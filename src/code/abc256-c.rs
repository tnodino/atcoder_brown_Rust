// https://atcoder.jp/contests/abc256/tasks/abc256_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        h1: isize,
        h2: isize,
        h3: isize,
        w1: isize,
        w2: isize,
        w3: isize,
    }
    let mut ans = 0;
    for a1 in 1..=30 {
        for a2 in 1..=30 {
            for b1 in 1..=30 {
                for b2 in 1..=30 {
                    if a1 + a2 >= h1 {
                        continue;
                    }
                    if b1 + b2 >= h2 {
                        continue;
                    }
                    if a1 + b1 >= w1 {
                        continue;
                    }
                    if a2 + b2 >= w2 {
                        continue;
                    }
                    let a3 = h1 - a1 - a2;
                    let b3 = h2 - b1 - b2;
                    let c1 = w1 - a1 - b1;
                    let c2 = w2 - a2 - b2;
                    let c3 = w3 - a3 - b3;
                    if c3 >= 1 && c1 + c2 + c3 == h3 {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}