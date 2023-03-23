// https://atcoder.jp/contests/arc003/tasks/arc003_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut s = Vec::new();
    for _ in 0..N {
        input! {
            v: String,
        }
        s.push((v.chars().rev().collect::<String>(), v));
    }
    s.sort_by(|a, b| a.0.cmp(&b.0));
    for i in 0..N {
        println!("{}", s[i].1);
    }
}