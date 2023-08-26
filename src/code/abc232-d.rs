// https://atcoder.jp/contests/abc232/tasks/abc232_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut C = Vec::new();
    for _ in 0..H {
        input! {
            c: String,
        }
        let c = c.chars().collect::<Vec<char>>();
        C.push(c);
    }
    let mut cost = vec![vec![0; W+1]; H+1];
    for i in (0..H).rev() {
        for j in (0..W).rev() {
            if C[i][j] == '#' {
                continue;
            }
            cost[i][j] = max(cost[i+1][j], cost[i][j+1]) + 1;
        }
    }
    println!("{}", cost[0][0]);
}