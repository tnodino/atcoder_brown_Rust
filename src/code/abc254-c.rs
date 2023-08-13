// https://atcoder.jp/contests/abc254/tasks/abc254_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut a: [usize; N],
    }
    for i in 0..K {
        let mut c = Vec::new();
        for j in (i..N).step_by(K) {
            c.push(a[j]);
        }
        c.sort();
        for j in 0..c.len() {
            a[j*K+i] = c[j];
        }
    }
    for i in 0..N-1 {
        if a[i] > a[i+1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}