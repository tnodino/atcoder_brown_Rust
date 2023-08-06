// https://atcoder.jp/contests/abc256/tasks/abc256_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            L: usize,
            R: usize,
        }
        vec.push((L, R));
    }
    vec.sort();
    let mut L = vec[0].0;
    let mut R = vec[0].1;
    for i in 1..N {
        if vec[i].0 > R {
            println!("{} {}", L, R);
            L = vec[i].0;
            R = vec[i].1;
        }
        else {
            R = max(R, vec[i].1);
        }
    }
    println!("{} {}", L, R);
}