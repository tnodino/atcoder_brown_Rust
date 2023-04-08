// https://atcoder.jp/contests/abc167/tasks/abc167_c

use proconio::input;
use proconio::fastout;
use std::usize::MAX;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: usize,
    }
    let mut C = Vec::new();
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            c: usize,
            a: [usize; M],
        }
        C.push(c);
        A.push(a);
    }
    let mut ans = MAX;
    for bit in 0..1<<N {
        let mut res = 0;
        let mut know = vec![0; M];
        for i in 0..N {
            if bit & (1 << i) > 0 {
                res += C[i];
                for j in 0..M {
                    know[j] += A[i][j];
                }
            }
        }
        for i in 0..M {
            if know[i] < X {
                res = MAX;
            }
        }
        ans = min(ans, res);
    }
    if ans == MAX {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}