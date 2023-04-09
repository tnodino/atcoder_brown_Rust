// https://atcoder.jp/contests/agc006/tasks/agc006_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        s: String,
        t: String,
    }
    for i in 0..=N {
        if &s[i..] == &t[..N-i] {
            println!("{}", 2 * N - (N - i));
            return;
        }
    }
}