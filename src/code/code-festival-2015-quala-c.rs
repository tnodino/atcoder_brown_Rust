// https://atcoder.jp/contests/code-festival-2015-quala/tasks/codefestival_2015_qualA_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(a - b);
    }
    let mut su = A.iter().sum::<usize>();
    if su <= T {
        println!("0");
        return;
    }
    B.sort_by(|a, b| b.cmp(a));
    for i in 0..N {
        su -= B[i];
        if su <= T {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}