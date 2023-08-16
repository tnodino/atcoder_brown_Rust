// https://atcoder.jp/contests/abc238/tasks/abc238_c

use proconio::input;
use proconio::fastout;
use num::pow;

const MOD: usize = 998_244_353;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut ans = 0;
    for i in 0..=18 {
        let l = pow(10 as usize, i);
        let r = pow(10 as usize, i+1);
        if N < l {
            break;
        }
        let x;
        if r <= N {
            x = 9 * pow(10 as usize, i) % MOD;
        }
        else {
            x = (N + 1 - l) % MOD;
        }
        ans += (x + 1) * x % MOD * bin_power(2, MOD-2) % MOD;
        ans %= MOD;
    }
    println!("{}", ans);
}