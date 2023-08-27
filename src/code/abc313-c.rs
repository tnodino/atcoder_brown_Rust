// https://atcoder.jp/contests/abc313/tasks/abc313_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
    }
    let M = N as isize;
    let s = A.iter().sum::<isize>();
    let t = s as usize;
    let mut B = vec![s/M; N];
    for i in 0..t%N {
        B[N-i-1] += 1;
    }
    A.sort();
    let mut ans = 0;
    for i in 0..N {
        ans += (A[i] - B[i]).abs();
    }
    println!("{}", ans / 2);
}