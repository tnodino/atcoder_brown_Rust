// https://atcoder.jp/contests/abc092/tasks/arc093_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let A = [&[0], &A[..], &[0]].concat();
    let mut B = vec![0; N+1];
    let mut s = 0;
    for i in 0..=N {
        let cost = (A[i+1] - A[i]).abs();
        s += cost;
        B[i] = cost;
    }
    for i in 0..N {
        println!("{}", s - B[i] - B[i+1] + (A[i+2] - A[i]).abs())
    }
}