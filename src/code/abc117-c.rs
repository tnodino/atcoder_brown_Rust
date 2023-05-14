// https://atcoder.jp/contests/abc117/tasks/abc117_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut X: [isize; M],
    }
    if N >= M {
        println!("0");
        return;
    }
    X.sort();
    let mut vec = Vec::new();
    for i in 0..M-1 {
        vec.push(X[i+1] - X[i]);
    }
    vec.sort();
    let mut ans = 0;
    for i in 0..M-N {
        ans += vec[i];
    }
    println!("{}", ans);
}