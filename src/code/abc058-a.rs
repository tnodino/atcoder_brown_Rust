// https://atcoder.jp/contests/abc058/tasks/arc071_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut cnt = [100; 26];
    for _ in 0..n {
        input! {
            S: String,
        }
        let mut cnt2 = [0; 26];
        for s in S.chars() {
            cnt2[(s as usize)-97] += 1;
        }
        for i in 0..26 {
            cnt[i] = min(cnt[i], cnt2[i]);
        }
    }
    for i in 0..26 {
        for _ in 0..cnt[i] {
            print!("{}", (i as u8 + 97) as char);
        }
    }
    println!();
}