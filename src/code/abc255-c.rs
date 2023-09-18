// https://atcoder.jp/contests/abc255/tasks/abc255_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

fn f(x: isize, a: isize, d: isize) -> isize {
    return a + d * x;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (X, mut A, mut D, N): (isize, isize, isize, isize),
    }
    if D < 0 {
        A = f(N - 1, A, D);
        D = -D;
    }
    let mut l = 0;
    let mut r = N - 1;
    for _ in 0..100 {
        let mid = (l + r) / 2;
        let res = f(mid, A, D);
        if res <= X {
            l = mid;
        }
        else {
            r = mid;
        }
    }
    println!("{}", min((X - f(l, A, D)).abs(), (X - f(r, A, D)).abs()));
}