// https://atcoder.jp/contests/arc014/tasks/arc014_2

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut set = HashSet::new();
    let mut rev = "".to_string();
    for i in 0..N {
        input! {
            W: String,
        }
        if i > 0 {
            if set.contains(&W) || rev.chars().last() != W.chars().next() {
                if i % 2 == 0 {
                    println!("LOSE");
                }
                else {
                    println!("WIN");
                }
                return;
            }
        }
        set.insert(W.clone());
        rev = W;
    }
    println!("DRAW");
}