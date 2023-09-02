// https://atcoder.jp/contests/arc161/tasks/arc161_b

use proconio::input;
use itertools::Itertools;

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[allow(non_snake_case)]
fn solve(vec: &Vec<usize>) {
    input! {
        N: usize,
    }
    if N <= 6 {
        println!("-1");
    }
    else {
        println!("{}", vec[bisect_left(&vec, &(N+1))-1]);
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    let mut vec = Vec::new();
    for perm in (0..60).combinations(3) {
        let mut x = 0;
        for i in 0..3 {
            x += 1 << perm[i];
        }
        vec.push(x);
    }
    vec.sort();
    for _ in 0..T {
        solve(&vec);
    }
}