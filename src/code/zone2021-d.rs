// https://atcoder.jp/contests/zone2021/tasks/zone2021_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut N = 0;
    let mut que = VecDeque::new();
    let mut flg = true;
    for s in S.chars() {
        if s == 'R' {
            flg ^= true;
        }
        else {
            N += 1;
            if flg {
                que.push_back(s);
                if 2 <= N && que[N-1] == que[N-2] {
                    que.pop_back();
                    que.pop_back();
                    N -= 2;
                }
            }
            else {
                que.push_front(s);
                if 2 <= N && que[0] == que[1] {
                    que.pop_front();
                    que.pop_front();
                    N -= 2;
                }
            }
        }
    }
    let mut S = que.into_iter().collect::<String>();
    if !flg {
        S = S.chars().rev().collect::<String>();
    }
    println!("{}", S);
}