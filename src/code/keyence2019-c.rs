// https://atcoder.jp/contests/keyence2019/tasks/keyence2019_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }
    let mut p = Vec::new();
    let mut cnt = 0;
    let mut minus = 0;
    for i in 0..N {
        if A[i] >= B[i] {
            p.push(A[i] - B[i]);
        }
        else {
            cnt += 1;
            minus += B[i] - A[i];
        }
    }
    p.sort_by(|a, b| b.cmp(a));
    let M = p.len();
    let mut P = vec![0; M+1];
    for i in 0..M {
        P[i+1] += p[i] + P[i];
    }
    for i in 0..=M {
        if minus <= P[i] {
            println!("{}", cnt + i);
            return;
        }
    }
    println!("-1");
}