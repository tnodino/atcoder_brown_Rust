// https://atcoder.jp/contests/abc212/tasks/abc212_d

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut bh = BinaryHeap::new();
    let mut dif = 0;
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                X: isize,
            }
            bh.push(-(X - dif));
        }
        else if q == 2 {
            input! {
                X: isize,
            }
            dif += X;
        }
        else {
            let res = -(bh.pop().unwrap()) + dif;
            println!("{}", res);
        }
    }
}