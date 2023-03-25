// https://atcoder.jp/contests/abc008/tasks/abc008_2

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..N {
        input! {
            S: String,
        }
        *map.entry(S).or_insert(0) += 1;
    }
    let ma = map.values().max().unwrap();
    for (k, v) in map.iter() {
        if ma == v {
            println!("{}", k);
            return;
        }
    }
}