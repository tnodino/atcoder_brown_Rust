// https://atcoder.jp/contests/abc286/tasks/abc286_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    let mut ans = 1<<60;
    for i in 0..N {
        let mut res = A * i;
        for j in 0..N/2 {
            if S[j] != S[N-j-1] {
                res += B;
            }
        }
        ans = min(ans, res);
        let tmp = S.remove(0);
        S.push(tmp);
    }
    println!("{}", ans);
}