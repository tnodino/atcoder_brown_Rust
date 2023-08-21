// https://atcoder.jp/contests/abc265/tasks/abc265_d

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
        P: usize,
        Q: usize,
        R: usize,
        A: [usize; N],
    }
    let mut S = vec![0; N+1];
    for i in 0..N {
        S[i+1] = S[i] + A[i];
    }
    for x in 0..N {
        let s = S[x] + P;
        let y = bisect_left(&S, &s);
        if y == N + 1 || S[y] != s {
            continue;
        }
        let s = S[y] + Q;
        let z = bisect_left(&S, &s);
        if z == N + 1 || S[z] != s {
            continue;
        }
        let s = S[z] + R;
        let w = bisect_left(&S, &s);
        if w == N + 1 || S[w] != s {
            continue;
        }
        println!("Yes");
        return;
    }
    println!("No");
}