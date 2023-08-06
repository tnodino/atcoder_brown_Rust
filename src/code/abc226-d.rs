// https://atcoder.jp/contests/abc226/tasks/abc226_d

use proconio::input;
use proconio::fastout;
use num::integer::gcd;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            x: isize,
            y: isize,
        }
        vec.push((x, y));
    }
    let mut set = HashSet::new();
    for i in 0..N {
        for j in 0..N {
            if i == j {
                continue;
            }
            let mut x = vec[i].0 - vec[j].0;
            let mut y = vec[i].1 - vec[j].1;
            let g = gcd(x, y);
            x /= g;
            y /= g;
            set.insert((x, y));
        }
    }
    println!("{}", set.len());
}