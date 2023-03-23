// https://atcoder.jp/contests/abc283/tasks/abc283_d

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut idx = 0;
    let mut flg = vec![HashSet::new(); S.len()];
    for s in S.chars() {
        if s == '(' {
            idx += 1;
            flg[idx] = flg[idx-1].clone();
        }
        else if s == ')' {
            flg[idx] = HashSet::new();
            idx -= 1;
        }
        else {
            if flg[idx].contains(&s) {
                println!("No");
                return;
            }
            flg[idx].insert(s);
        }
    }
    println!("Yes");
}