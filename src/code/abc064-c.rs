// https://atcoder.jp/contests/abc064/tasks/abc064_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut cnt = [0; 9];
    for i in 0..N {
        cnt[min(8, a[i]/400)] += 1;
    }
    let mut ans = 0;
    for i in 0..8 {
        if cnt[i] >= 1 {
            ans += 1;
        }
    }
    if cnt[8] == 0 {
        println!("{} {}", ans, ans);
    }
    else {
        if ans == 0 {
            println!("{} {}", 1, cnt[8]);
        }
        else {
            println!("{} {}", ans, ans + cnt[8]);
        }
    }
}