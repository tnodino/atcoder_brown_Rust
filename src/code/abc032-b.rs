// https://atcoder.jp/contests/abc032/tasks/abc032_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
        k: usize,
    }
    let N = s.len();
    if N < k {
        println!("0");
        return
    }
    let mut set = HashSet::new();
    for i in 0..=N-k {
        set.insert(&s[i..i+k]);
    }
    println!("{}", set.len());
}