// https://atcoder.jp/contests/abc252/tasks/abc252_c

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut ans = 1<<32;
    for x in 0..10 {
        let mut cnt = [0; 10];
        for i in 0..N {
            for j in 0..10 {
                if S[i][j] as usize - 48 == x {
                    cnt[j] += 1;
                }
            }
        }
        let mut res = 0;
        for i in 0..10 {
            if cnt[i] >= 1 {
                res = max(res, (cnt[i] - 1) * 10 + i);
            }
        }
        ans = min(ans, res);
    }
    println!("{}", ans);
}