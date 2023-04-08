// https://atcoder.jp/contests/abc159/tasks/abc159_d

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..N {
        *map.entry(A[i]).or_insert(0) += 1;
    }
    let mut cnt = 0;
    for v in map.values() {
        cnt += (v - 1) * v / 2;
    }
    for i in 0..N {
        println!("{}", cnt - map.get(&A[i]).unwrap() + 1);
    }
}