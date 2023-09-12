// https://atcoder.jp/contests/abc205/tasks/abc205_d

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
        (N, Q): (usize, usize),
        A: [usize; N],
    }
    let mut cnt = vec![0; N];
    cnt[0] = A[0] - 1;
    for i in 1..N {
        cnt[i] = cnt[i-1] + (A[i] - A[i-1]) - 1;
    }
    for _ in 0..Q {
        input! {
            K: usize,
        }
        let idx = bisect_left(&cnt, &K);
        println!("{}", K + idx);
    }
}