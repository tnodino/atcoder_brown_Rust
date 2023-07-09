// https://atcoder.jp/contests/arc117/tasks/arc117_b

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort();
    let mut ans = A[0] + 1;
    for i in 1..N {
        ans *= A[i] - A[i-1] + 1;
        ans %= MOD;
    }
    println!("{}", ans);
}