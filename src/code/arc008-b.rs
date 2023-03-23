// https://atcoder.jp/contests/arc008/tasks/arc008_2

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        _M: usize,
        name: String,
        kit: String,
    }
    let mut cnt1 = [0; 26];
    let mut cnt2 = [0; 26];
    for c in name.chars() {
        cnt1[c as usize - 65] += 1;
    }
    for c in kit.chars() {
        cnt2[c as usize - 65] += 1;
    }
    let mut ans = 0;
    for i in 0..26 {
        if cnt1[i] == 0 {
            continue;
        }
        if cnt2[i] == 0 {
            println!("-1");
            return;
        }
        ans = max(ans, (cnt1[i] + cnt2[i] - 1) / cnt2[i]);
    }
    println!("{}", ans);
}