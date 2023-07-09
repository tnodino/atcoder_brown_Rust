// https://atcoder.jp/contests/abc220/tasks/abc220_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut DP = vec![vec![0; 10]; N];
    DP[0][A[0]] = 1;
    for i in 1..N {
        for j in 0..10 {
            DP[i][(j+A[i])%10] += DP[i-1][j];
            DP[i][(j*A[i])%10] += DP[i-1][j];
        }
        for j in 0..10 {
            DP[i][j] %= MOD;
        }
    }
    for j in 0..10 {
        println!("{}", DP[N-1][j]);
    }
}