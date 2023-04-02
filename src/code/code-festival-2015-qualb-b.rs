// https://atcoder.jp/contests/code-festival-2015-qualb/tasks/codefestival_2015_qualB_b

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        _M: usize,
        A: [usize; N],
    }
    let mut map = HashMap::new();
    for a in A {
        *map.entry(a).or_insert(0) += 1;
    }
    let L = (N + 2) / 2;
    for (k, v) in map {
        if v >= L {
            println!("{}", k);
            return;
        }
    }
    println!("?");
}