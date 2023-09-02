// https://atcoder.jp/contests/arc139/tasks/arc139_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: [usize; N],
    }
    let mut A: Vec<usize> = vec![0; N];
    A[0] = 1 << T[0];
    for i in 1..N {
        let mut x = ((A[i-1] >> T[i]) + 1) << T[i];
        if (x >> T[i]) & 1 == 0 {
            x += 1 << T[i];
        }
        A[i] = x;
    }
    println!("{}", A[N-1]);
}