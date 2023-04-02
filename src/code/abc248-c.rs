// https://atcoder.jp/contests/abc248/tasks/abc248_c

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }
    let mut DP = vec![vec![0; K+1]; N+1];
    DP[0][0] = 1;
    for i in 0..N {
        for k in 0..K {
            for a in 1..=M {
                if k + a > K {
                    break;
                }
                DP[i+1][k+a] += DP[i][k];
                DP[i+1][k+a] %= MOD;
            }
        }
    }
    let mut ans = 0;
    for i in 0..=K {
        ans += DP[N][i];
        ans %= MOD;
    }
    println!("{}", ans);
}