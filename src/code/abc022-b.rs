// https://atcoder.jp/contests/abc022/tasks/abc022_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut set = HashSet::new();
    let mut ans = 0;
    for a in A {
        if set.contains(&a) {
            ans += 1;
        }
        else {
            set.insert(a);
        }
    }
    println!("{}", ans);
}