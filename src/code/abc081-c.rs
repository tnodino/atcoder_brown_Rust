// https://atcoder.jp/contests/abc081/tasks/arc086_a

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }
    let mut map = HashMap::new();
    for i in 0..N {
        *map.entry(A[i]).or_insert(0) += 1;
    }
    if map.len() <= K {
        println!("0");
        return;
    }
    let mut vec = map.values().collect::<Vec<_>>();
    vec.sort();
    let M = vec.len();
    let mut ans = 0;
    for i in 0..M-K {
        ans += vec[i];
    }
    println!("{}", ans);
}