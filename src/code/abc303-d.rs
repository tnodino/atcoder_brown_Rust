// https://atcoder.jp/contests/abc303/tasks/abc303_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut DP: Vec<Vec<usize>> = vec![vec![1<<60; 2]; N+1];
    DP[0][0] = 0;
    for i in 0..N {
        if S[i] == 'a' {
            DP[i+1][0] = min(DP[i][0] + X, DP[i][1] + Z + X);
            DP[i+1][1] = min(DP[i][1] + Y, DP[i][0] + Z + Y);
        }
        else {
            DP[i+1][0] = min(DP[i][0] + Y, DP[i][1] + Z + Y);
            DP[i+1][1] = min(DP[i][1] + X, DP[i][0] + Z + X);
        }
    }
    println!("{}", min(DP[N][0], DP[N][1]));
}