// https://atcoder.jp/contests/abc233/tasks/abc233_d

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }
    let mut S = vec![0; N+1];
    for i in 0..N {
        S[i+1] = S[i] + A[i];
    }
    let mut map: HashMap<isize, usize> = HashMap::new();
    let mut ans: usize = 0;
    for i in 1..=N {
        *map.entry(S[i-1]).or_insert(0) += 1;
        ans += map.get(&(S[i]-K)).map_or(0, |&v| v);
    }
    println!("{}", ans);
}