// https://atcoder.jp/contests/abc051/tasks/abc051_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        S: usize,
    }
    let mut ans = 0;
    for x in 0..=K {
        for y in 0..=K {
            if x + y > S {
                break;
            }
            if S - (x + y) <= K {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}