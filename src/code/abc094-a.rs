// https://atcoder.jp/contests/abc094/tasks/arc095_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut B = A.clone();
    B.sort();
    let mid = (B[N/2-1], B[N/2]);
    for i in 0..N {
        if A[i] <= mid.0 {
            println!("{}", mid.1);
        }
        else {
            println!("{}", mid.0);
        }
    }
}