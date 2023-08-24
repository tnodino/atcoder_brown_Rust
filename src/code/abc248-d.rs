// https://atcoder.jp/contests/abc248/tasks/abc248_d

use proconio::input;
use proconio::fastout;

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

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut vec = vec![Vec::new(); N+1];
    for i in 0..N {
        vec[A[i]].push(i+1);
    }
    input! {
        Q: usize,
    }
    for _ in 0..Q {
        input! {
            L: usize,
            R: usize,
            X: usize,
        }
        println!("{}", bisect_left(&vec[X], &(R+1)) - bisect_left(&vec[X], &L));
    }
}