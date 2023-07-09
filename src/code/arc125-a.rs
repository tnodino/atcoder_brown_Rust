// https://atcoder.jp/contests/arc125/tasks/arc125_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: [usize; N],
        mut T: [usize; M],
    }
    let mut idx = vec![N, N];
    for i in 0..N {
        if S[i] == 0 {
            idx[0] = min(idx[0], i);
        }
        else {
            idx[1] = min(idx[1], i);
        }
    }
    for i in (0..N).rev() {
        if S[i] == 0 {
            idx[0] = min(idx[0], N - i);
        }
        else {
            idx[1] = min(idx[1], N - i);
        }
    }
    T.dedup();
    let x = idx[S[0] ^ 1];
    if T.len() == 1 {
        if idx[T[0]] == N {
            println!("-1");
        }
        else if S[0] == T[0] {
            println!("{}", M);
        }
        else {
            println!("{}", x + M);
        }
    }
    else {
        if idx[0] == N || idx[1] == N {
            println!("-1");
        }
        else if S[0] == T[0] {
            println!("{}", x + (T.len() - 2) + M);
        }
        else {
            println!("{}", x + (T.len() - 1) + M);
        }
    }
}