// https://atcoder.jp/contests/abc129/tasks/abc129_c

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg = vec![false; N+1];
    for _ in 0..M {
        input! {
            a: usize,
        }
        flg[a] = true;
    }
    let mut DP = vec![0; N+1];
    DP[0] = 1;
    for i in 0..=N {
        if flg[i] {
            continue;
        }
        if 0 < i {
            DP[i] += DP[i-1];
        }
        if 1 < i {
            DP[i] += DP[i-2];
        }
        DP[i] %= MOD;
    }
    println!("{}", DP[N]);
}