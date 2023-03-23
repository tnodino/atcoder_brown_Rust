// https://atcoder.jp/contests/abc289/tasks/abc289_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        M: usize,
        B: [usize; M],
        X: usize,
    }
    let mut flg = vec![false; X+1];
    for b in B {
        flg[b] = true;
    }
    let mut DP = vec![false; X+1];
    DP[0] = true;
    for i in 1..=X {
        if flg[i] {
            continue;
        }
        for j in 0..N {
            if A[j] <= i {
                DP[i] |= DP[i - A[j]];
            }
        }
    }
    if DP[X] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}