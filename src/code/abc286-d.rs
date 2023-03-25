// https://atcoder.jp/contests/abc286/tasks/abc286_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
    }
    let mut DP = vec![vec![false; X+1]; N+1];
    DP[0][0] = true;
    for i in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        for j in 0..=X {
            for k in 0..=B {
                if A * k <= j {
                    DP[i+1][j] |= DP[i][j-A*k];
                }
            }
        }
    }
    if DP[N][X] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}