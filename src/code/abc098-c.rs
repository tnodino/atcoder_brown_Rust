// https://atcoder.jp/contests/abc098/tasks/arc098_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut e = vec![0; N];
    let mut w = vec![0; N];
    for i in (1..N).rev() {
        e[i-1] += e[i];
        if S[i] == 'E' {
            e[i-1] += 1;
        }
    }
    for i in 0..N-1 {
        w[i+1] += w[i];
        if S[i] == 'W' {
            w[i+1] += 1;
        }
    }
    let mut ans = N;
    for i in 0..N {
        ans = min(ans, w[i] + e[i]);
    }
    println!("{}", ans);
}