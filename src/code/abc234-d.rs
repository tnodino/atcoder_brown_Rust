// https://atcoder.jp/contests/abc234/tasks/abc234_d

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        P: [usize; N],
    }
    let mut bh = BinaryHeap::new();
    for i in 0..K {
        bh.push(Reverse(P[i]));
    }
    let x = bh.pop().unwrap().0;
    println!("{}", x);
    bh.push(Reverse(x));
    for i in K..N {
        let x = max(bh.pop().unwrap().0, P[i]);
        bh.push(Reverse(x));
        let x = bh.pop().unwrap().0;
        println!("{}", x);
        bh.push(Reverse(x));
    }
}