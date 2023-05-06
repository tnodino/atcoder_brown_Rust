// https://atcoder.jp/contests/abc071/tasks/arc081_a

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut map = HashMap::new();
    for i in 0..N {
        *map.entry(A[i]).or_insert(0) += 1;
    }
    let mut ans = 0;
    let mut vec = Vec::new();
    for (k, v) in map.iter() {
        if *v >= 4 {
            ans = max(ans, k * k);
        }
        if *v >= 2 {
            vec.push(*k);
        }
    }
    vec.sort_by(|a, b| b.cmp(a));
    if vec.len() >= 2 {
        ans = max(ans, vec[0] * vec[1]);
    }
    println!("{}", ans);
}