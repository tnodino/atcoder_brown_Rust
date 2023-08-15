// https://atcoder.jp/contests/abc247/tasks/abc247_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut que = VecDeque::new();
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: usize,
                c: usize,
            }
            que.push_back((x, c));
        }
        else {
            input! {
                mut c: usize,
            }
            let mut ans = 0;
            loop {
                let (a, b) = que.pop_front().unwrap();
                if c <= b {
                    ans += a * c;
                    que.push_front((a, b-c));
                    break;
                }
                else {
                    ans += a * b;
                    c -= b;
                }
            }
            println!("{}", ans);
        }
    }
}