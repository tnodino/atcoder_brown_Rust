// https://atcoder.jp/contests/arc039/tasks/arc039_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: isize,
        mut B: isize,
    }
    let mut vec = Vec::new();
    for _ in 0..3 {
        vec.push(B % 10);
        B /= 10;
    }
    for _ in 0..3 {
        vec.push(A % 10);
        A /= 10;
    }
    vec.reverse();
    let mut ans = -100;
    for i in 0..6 {
        let mut A = 0;
        for k in 0..3 {
            A *= 10;
            if k == i {
                A += 9;
            }
            else {
                A += vec[k];
            }
        }
        let mut B = 0;
        for k in 0..3 {
            B *= 10;
            if k + 3 == i {
                if k == 0 {
                    B += 1;
                }
            }
            else {
                B += vec[k+3];
            }
        }
        ans = max(ans, A - B);
    }
    println!("{}", ans);
}