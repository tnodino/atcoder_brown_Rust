// https://atcoder.jp/contests/arc033/tasks/arc033_2

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        NA: usize,
        NB: usize,
        A: [usize; NA],
        B: [usize; NB],
    }
    let mut set = HashSet::new();
    for a in A {
        set.insert(a);
    }
    let mut cnt = 0;
    for b in B {
        if set.contains(&b) {
            cnt += 1;
        }
        set.insert(b);
    }
    println!("{}", cnt as f64 / set.len() as f64);
}