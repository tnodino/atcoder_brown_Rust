// https://atcoder.jp/contests/abc218/tasks/abc218_d

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    let mut set = HashSet::new();
    for _ in 0..N {
        input! {
            x: usize,
            y: usize,
        }
        vec.push((x, y));
        set.insert((x, y));
    }
    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            if vec[i].0 >= vec[j].0 || vec[i].1 >= vec[j].1 {
                continue;
            }
            if set.contains(&(vec[i].0, vec[j].1)) && set.contains(&(vec[j].0, vec[i].1)) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}