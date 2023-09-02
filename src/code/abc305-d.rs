// https://atcoder.jp/contests/abc305/tasks/abc305_d

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

#[allow(non_snake_case)]
fn f(x: usize, A: &Vec<usize>, S: &Vec<usize>) -> usize {
    let idx = bisect_left(&A, &(x+1));
    let mut ret = S[idx];
    if idx % 2 == 0 {
        ret += x - A[idx-1];
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
    }
    let mut S = vec![0; N+1];
    for i in 1..N {
        S[i+1] += S[i];
        if i % 2 == 0 {
            S[i+1] += A[i] - A[i-1];
        }
    }
    for _ in 0..Q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", f(r, &A, &S) - f(l, &A, &S));
    }
}