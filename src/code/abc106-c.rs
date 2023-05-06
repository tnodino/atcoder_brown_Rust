// https://atcoder.jp/contests/abc106/tasks/abc106_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        K: usize,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in 0..K {
        if S[i] != '1' {
            println!("{}", S[i]);
            return;
        }
    }
    println!("1");
}