// https://atcoder.jp/contests/abc041/tasks/abc041_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut map = HashMap::new();
    for (i, v) in a.iter().enumerate() {
        *map.entry(v).or_insert(0) = i + 1;
    }
    let mut b = a.clone();
    b.sort_by(|a, b| b.cmp(a));
    for v in b {
        println!("{}", map.get(&v).unwrap());
    }
}