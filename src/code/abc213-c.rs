// https://atcoder.jp/contests/abc213/tasks/abc213_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _H: usize,
        _W: usize,
        N: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    let mut X = Vec::new();
    let mut Y = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
        X.push(a);
        Y.push(b);
    }
    X.sort();
    Y.sort();
    X.dedup();
    Y.dedup();
    let mut C = HashMap::new();
    let mut D = HashMap::new();
    for i in 0..X.len() {
        C.insert(X[i], i+1);
    }
    for i in 0..Y.len() {
        D.insert(Y[i], i+1);
    }
    for i in 0..N {
        println!("{} {}", C[&A[i]], D[&B[i]]);
    }
}