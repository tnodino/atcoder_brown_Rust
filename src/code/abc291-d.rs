// https://atcoder.jp/contests/abc291/tasks/abc291_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    let mut DP = vec![vec![0; 2]; N];
    DP[0][0] = 1;
    DP[0][1] = 1;
    for i in 1..N {
        if A[i] != A[i-1] {
            DP[i][0] += DP[i-1][0];
        }
        if A[i] != B[i-1] {
            DP[i][0] += DP[i-1][1];
        }
        if B[i] != A[i-1] {
            DP[i][1] += DP[i-1][0];
        }
        if B[i] != B[i-1] {
            DP[i][1] += DP[i-1][1];
        }
        DP[i][0] %= MOD;
        DP[i][1] %= MOD;
    }
    println!("{}", (DP[N-1][0] + DP[N-1][1]) % MOD);
}