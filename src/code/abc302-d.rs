// https://atcoder.jp/contests/abc302/tasks/abc302_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        D: isize,
        mut A: [isize; N],
        mut B: [isize; M],
    }
    A.sort_by(|a, b| b.cmp(a));
    B.sort_by(|a, b| b.cmp(a));
    let mut aidx = 0;
    let mut bidx = 0;
    while aidx < N && bidx < M {
        if (A[aidx] - B[bidx]).abs() <= D {
            println!("{}", A[aidx] + B[bidx]);
            return;
        }
        if A[aidx] > B[bidx] {
            aidx += 1;
        }
        else {
            bidx += 1;
        }
    }
    println!("-1");
}