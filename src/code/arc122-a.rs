// https://atcoder.jp/contests/arc122/tasks/arc122_a

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut DP = vec![vec![0; 2]; N+1];
    let mut p = 1;
    let mut m = 0;
    for i in 0..N {
        DP[i+1][0] = (DP[i][0] + DP[i][1] + (A[i] * p)) % MOD;
        DP[i+1][1] = (DP[i][0] + MOD - ((A[i] * m) % MOD)) % MOD;
        let a = p + m;
        let b = p;
        p = a % MOD;
        m = b % MOD;
    }
    println!("{}", (DP[N][0] + DP[N][1]) % MOD);
}