// https://atcoder.jp/contests/abc063/tasks/arc075_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 10000;
    let mut DP = vec![vec![false; M+1]; N+1];
    DP[0][0] = true;
    for i in 0..N {
        input! {
            s: usize,
        }
        for j in 0..=M {
            DP[i+1][j] |= DP[i][j];
            if j + s <= M {
                DP[i+1][j+s] |= DP[i][j];
            }
        }
    }
    for i in (0..=M).rev() {
        if DP[N][i] && i % 10 > 0 {
            println!("{}", i);
            return;
        }
    }
    println!("0");
}