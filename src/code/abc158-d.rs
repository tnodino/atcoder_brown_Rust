// https://atcoder.jp/contests/abc158/tasks/abc158_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        Q: usize,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut S = VecDeque::from(S);
    let mut flg = 0;
    for _ in 0..Q {
        input! {
            T: usize,
        }
        if T == 1 {
            flg ^= 1;
        }
        else {
            input! {
                F: usize,
                C: char,
            }
            if flg ^ (F - 1) == 0 {
                S.push_front(C);
            }
            else {
                S.push_back(C);
            }
        }
    }
    let mut S = S.into_iter().collect::<Vec<char>>();
    if flg == 1 {
        S.reverse();
    }
    println!("{}", S.iter().map(|&x| x.to_string()).collect::<String>());
}