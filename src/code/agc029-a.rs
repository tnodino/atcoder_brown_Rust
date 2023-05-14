// https://atcoder.jp/contests/agc029/tasks/agc029_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut idx = 0;
    let mut ans = 0;
    for (i, s) in S.chars().enumerate() {
        if s == 'W' {
            ans += i - idx;
            idx += 1;
        }
    }
    println!("{}", ans);
}