// https://atcoder.jp/contests/abc149/tasks/abc149_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        R: usize,
        S: usize,
        P: usize,
        T: String,
    }
    let mut T = T.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..N {
        if i < K || T[i] != T[i-K] {
            ans += match T[i] {
                'r' => P,
                's' => R,
                _ => S,
            }
        }
        else {
            T[i] = '-';
        }
    }
    println!("{}", ans);
}