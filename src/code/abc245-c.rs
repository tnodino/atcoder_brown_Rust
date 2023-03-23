// https://atcoder.jp/contests/abc245/tasks/abc245_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
        B: [isize; N],
    }
    let mut DP = vec![vec![false; 2]; N];
    DP[0][0] = true;
    DP[0][1] = true;
    for i in 1..N {
        if (A[i-1] - A[i]).abs() <= K {
            DP[i][0] |= DP[i-1][0];
        }
        if (B[i-1] - A[i]).abs() <= K {
            DP[i][0] |= DP[i-1][1];
        }
        if (A[i-1] - B[i]).abs() <= K {
            DP[i][1] |= DP[i-1][0];
        }
        if (B[i-1] - B[i]).abs() <= K {
            DP[i][1] |= DP[i-1][1];
        }
    }
    if DP[N-1][0] || DP[N-1][1] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}