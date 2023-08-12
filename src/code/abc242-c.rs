// https://atcoder.jp/contests/abc242/tasks/abc242_c

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut DP = vec![vec![0; 10]; N];
    DP[0] = vec![1; 10];
    for i in 1..N {
        for j in 1..10 {
            if j > 1 {
                DP[i][j] += DP[i-1][j-1];
            }
            DP[i][j] += DP[i-1][j];
            if j < 9 {
                DP[i][j] += DP[i-1][j+1];
            }
            DP[i][j] %= MOD;
        }
    }
    println!("{}", DP[N-1].iter().sum::<usize>() % MOD);
}