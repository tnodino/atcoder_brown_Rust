// https://atcoder.jp/contests/agc009/tasks/agc009_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    let mut cnt = 0;
    for i in (0..N).rev() {
        if (A[i] + cnt) % B[i] == 0 {
            continue;
        }
        let m = B[i] - ((A[i] + cnt) % B[i]);
        cnt += m;
    }
    println!("{}", cnt);
}