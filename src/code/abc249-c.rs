// https://atcoder.jp/contests/abc249/tasks/abc249_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut ans = 0;
    for bit in 1..1<<N {
        let mut cnt = vec![0; 26];
        for i in 0..N {
            if bit & (1 << i) > 0 {
                for j in 0..S[i].len() {
                    let idx = (S[i][j] as usize) - ('a' as usize);
                    cnt[idx] += 1;
                }
            }
        }
        let mut res = 0;
        for i in 0..26 {
            if cnt[i] == K {
                res += 1;
            }
        }
        ans = max(ans, res);
    }
    println!("{}", ans);
}