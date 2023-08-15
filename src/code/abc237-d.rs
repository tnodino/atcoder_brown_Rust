// https://atcoder.jp/contests/abc237/tasks/abc237_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut que = VecDeque::new();
    que.push_back(N);
    for i in (0..N).rev() {
        if S[i] == 'L' {
            que.push_back(i);
        }
        else {
            que.push_front(i);
        }
    }
    println!("{}", que.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}