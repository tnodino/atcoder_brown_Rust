// https://atcoder.jp/contests/abc082/tasks/arc087_a

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
    for i in 0..N {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (k, v) in map {
        if k <= v {
            ans += v - k;
        }
        else {
            ans += v;
        }
    }
    println!("{}", ans);
}