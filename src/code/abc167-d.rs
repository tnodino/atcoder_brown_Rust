// https://atcoder.jp/contests/abc167/tasks/abc167_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }
    let M = 60;
    let mut mat = vec![vec![0; N]; M];
    for i in 0..N {
        mat[0][i] = A[i] - 1;
    }
    for i in 1..M {
        for j in 0..N {
            mat[i][j] = mat[i-1][mat[i-1][j]];
        }
    }
    let mut ans = 0;
    for i in 0..M {
        if K & (1 << i) > 0 {
            ans = mat[i][ans];
        }
    }
    println!("{}", ans + 1);
}