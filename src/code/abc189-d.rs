// https://atcoder.jp/contests/abc189/tasks/abc189_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut DP: Vec<Vec<usize>> = vec![vec![0; 2]; N+1];
    DP[0][0] = 1;
    DP[0][1] = 1;
    for i in 0..N {
        input! {
            S: String,
        }
        if S == "AND".to_string() {
            DP[i+1][0] += DP[i][0];
            DP[i+1][1] += DP[i][0] + DP[i][1] * 2;
        }
        else {
            DP[i+1][0] += DP[i][0] * 2 + DP[i][1];
            DP[i+1][1] += DP[i][1];
        }
    }
    println!("{}", DP[N][0]);
}