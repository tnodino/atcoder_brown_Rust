// https://atcoder.jp/contests/abc255/tasks/abc255_d

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
        Q: usize,
        mut A: [usize; N],
    }
    A.sort();
    let mut s = vec![0; N+1];
    for i in 0..N {
        s[i+1] = s[i] + A[i];
    }
    for _ in 0..Q {
        input! {
            X: usize,
        }
        let l = bisect_left(&A, &X);
        let r = bisect_left(&A, &(X+1));
        println!("{}", (X * l - s[l]) + ((s[N] - s[r]) - X * (N - r)));
    }
}