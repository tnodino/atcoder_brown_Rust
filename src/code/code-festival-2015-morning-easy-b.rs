// https://atcoder.jp/contests/code-festival-2015-morning-easy/tasks/cf_2015_morning_easy_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    if N % 2 == 1 {
        println!("-1");
        return;
    }
    let S = S.chars().collect::<Vec<char>>();
    let M = N / 2;
    let mut ans = 0;
    for i in 0..M {
        if S[i] != S[i+M] {
            ans += 1;
        }
    }
    println!("{}", ans);
}