// https://atcoder.jp/contests/abc190/tasks/abc190_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    input! {
        K: usize,
    }
    let mut C = Vec::new();
    let mut D = Vec::new();
    for _ in 0..K {
        input! {
            c: usize,
            d: usize,
        }
        C.push(c);
        D.push(d);
    }
    let mut ans = 0;
    for bit in 0..1<<K {
        let mut flg = vec![false; N+1];
        for i in 0..K {
            if bit & (1 << i) > 0 {
                flg[C[i]] = true;
            }
            else {
                flg[D[i]] = true;
            }
        }
        let mut res = 0;
        for i in 0..M {
            if flg[A[i]] && flg[B[i]] {
                res += 1;
            }
        }
        ans = max(ans, res);
    }
    println!("{}", ans);
}