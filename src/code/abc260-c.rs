// https://atcoder.jp/contests/abc260/tasks/abc260_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
    }
    let mut DP: Vec<Vec<usize>> = vec![vec![0; 2]; N+1];
    DP[N][0] = 1;
    for i in (2..=N).rev() {
        DP[i-1][0] += DP[i][0];
        DP[i][1] += DP[i][0] * X;
        DP[i-1][0] += DP[i][1];
        DP[i-1][1] += DP[i][1] * Y;
    }
    println!("{}", DP[1][1]);
}