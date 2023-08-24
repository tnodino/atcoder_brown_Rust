// https://atcoder.jp/contests/abc268/tasks/abc268_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut cnt = vec![0; N];
    for i in 0..N {
        for j in 0..3 {
            let idx = (P[i] + N + j - 1 - i) % N;
            cnt[idx] += 1;
        }
    }
    println!("{}", cnt.iter().max().unwrap());
}