// https://atcoder.jp/contests/abc072/tasks/arc082_a

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [isize; N],
    }
    let mut map = HashMap::new();
    for v in a {
        *map.entry(v-1).or_insert(0) += 1;
        *map.entry(v).or_insert(0) += 1;
        *map.entry(v+1).or_insert(0) += 1;
    }
    println!("{}", map.values().max().unwrap());
}