// https://atcoder.jp/contests/abc240/tasks/abc240_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X): (usize, usize),
    }
    let mut DP = vec![vec![false; X+1]; N+1];
    DP[0][0] = true;
    for i in 0..N {
        input! {
            (a, b): (usize, usize),
        }
        for j in 0..=X {
            if j + a <= X {
                DP[i+1][j+a] |= DP[i][j];
            }
            if j + b <= X {
                DP[i+1][j+b] |= DP[i][j];
            }
        }
    }
    println!("{}", match DP[N][X] {
        true => "Yes",
        false => "No",
    })
}