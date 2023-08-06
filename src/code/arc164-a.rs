// https://atcoder.jp/contests/arc164/tasks/arc164_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn solve() {
    input! {
        N: usize,
        K: usize,
    }
    let mut x = N;
    let mut cnt = 0;
    while x > 0 {
        cnt += x % 3;
        x /= 3;
    }
    if K < cnt {
        println!("No");
    }
    else if cnt % 2 == K % 2 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        solve();
    }
}