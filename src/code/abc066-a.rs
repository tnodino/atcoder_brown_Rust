// https://atcoder.jp/contests/abc066/tasks/arc077_a

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut b = VecDeque::new();
    for (i, v) in a.iter().enumerate() {
        if i % 2 == 0 {
            b.push_back(*v);
        }
        else {
            b.push_front(*v);
        }
    }
    let mut b = b.into_iter().collect::<Vec<usize>>();
    if n % 2 == 1 {
        b.reverse();
    }
    println!("{}", b.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}