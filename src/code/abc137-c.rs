// https://atcoder.jp/contests/abc137/tasks/abc137_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut map: HashMap<String, usize> = HashMap::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let mut s = s.chars().collect::<Vec<char>>();
        s.sort();
        let s = s.iter().collect::<String>();
        *map.entry(s).or_insert(0) += 1;
    }
    let mut ans = 0;
    for v in map.values() {
        ans += (v - 1) * v / 2;
    }
    println!("{}", ans);
}