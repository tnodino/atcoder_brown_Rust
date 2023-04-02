// https://atcoder.jp/contests/code-festival-2015-qualb/tasks/codefestival_2015_qualB_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N],
        mut B: [usize; M],
    }
    A.sort();
    B.sort();
    let mut idx = 0;
    for i in 0..N {
        if A[i] >= B[idx] {
            idx += 1;
            if idx == M {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}