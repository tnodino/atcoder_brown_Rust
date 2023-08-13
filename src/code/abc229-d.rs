// https://atcoder.jp/contests/abc229/tasks/abc229_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        K: usize,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut l = 0;
    let mut cnt = 0;
    let mut ans = 0;
    for r in 0..N {
        if S[r] == '.' {
            cnt += 1;
        }
        while cnt > K {
            if S[l] == '.' {
                cnt -= 1;
            }
            l += 1;
        }
        ans = max(ans, r - l + 1);
    }
    println!("{}", ans);
}