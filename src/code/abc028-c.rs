// https://atcoder.jp/contests/abc028/tasks/abc028_c

use proconio::input;
use proconio::fastout;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
        E: usize,
    }
    let arr = [A, B, C, D, E];
    let mut s = Vec::new();
    for comb in (0..5).combinations(3) {
        let mut x = 0;
        for i in 0..3 {
            x += arr[comb[i]];
        }
        s.push(x);
    }
    s.sort_by(|a, b| b.cmp(a));
    s.dedup();
    println!("{}", s[2]);
}