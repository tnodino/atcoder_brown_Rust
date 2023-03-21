// https://atcoder.jp/contests/abc294/tasks/abc294_e

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _L: usize,
        N1: usize,
        N2: usize,
    }
    let mut u1 = Vec::new();
    let mut l1 = Vec::new();
    for _ in 0..N1 {
        input! {
            u: usize,
            l: usize,
        }
        u1.push(u);
        l1.push(l);
    }
    let mut u2 = Vec::new();
    let mut l2 = Vec::new();
    for _ in 0..N2 {
        input! {
            u: usize,
            l: usize,
        }
        u2.push(u);
        l2.push(l);
    }
    for i in 1..N1 {
        l1[i] += l1[i-1];
    }
    for i in 1..N2 {
        l2[i] += l2[i-1];
    }
    let mut now = 0;
    let mut idx1 = 0;
    let mut idx2 = 0;
    let mut ans = 0;
    while idx1 < N1 && idx2 < N2 {
        let mi = min(l1[idx1], l2[idx2]);
        if u1[idx1] == u2[idx2] {
            ans += mi - now;
        }
        now = mi;
        if l1[idx1] == mi {
            idx1 += 1;
        }
        if l2[idx2] == mi {
            idx2 += 1;
        }
    }
    println!("{}", ans);
}