// https://atcoder.jp/contests/abc306/tasks/abc306_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut DP = vec![vec![0; 2]; N+1];
    for i in 0..N {
        input! {
            X: usize,
            Y: isize,
        }
        DP[i+1][0] = DP[i][0];
        DP[i+1][1] = DP[i][1];
        if X == 0 {
            DP[i+1][0] = max(DP[i+1][0], DP[i][0] + Y);
            DP[i+1][0] = max(DP[i+1][0], DP[i][1] + Y);
        }
        else {
            DP[i+1][1] = max(DP[i+1][1], DP[i][0] + Y);
        }
    }
    println!("{}", max(DP[N][0], DP[N][1]));
}