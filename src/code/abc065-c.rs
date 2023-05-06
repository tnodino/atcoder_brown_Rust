// https://atcoder.jp/contests/abc065/tasks/arc076_a

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
    let n = N as isize;
    let m = M as isize;
    if (n - m).abs() >= 2 {
        println!("0");
        return;
    }
    let K = 100_000;
    let mut fac = vec![1; K+1];
    for i in 1..=K {
        fac[i] *= fac[i-1] * i;
        fac[i] %= MOD;
    }
    if (n - m).abs() == 1 {
        println!("{}", fac[N] * fac[M] % MOD);
    }
    else {
        println!("{}", fac[N] * fac[M] * 2 % MOD);
    }
}