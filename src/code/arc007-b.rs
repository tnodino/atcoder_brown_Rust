// https://atcoder.jp/contests/arc007/tasks/arc007_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut ans = Vec::new();
    for i in 1..=N {
        ans.push(i);
    }
    let mut now = 0;
    let mut ans = Vec::new();
    for i in 1..=N {
        ans.push(i);
    }
    for _ in 0..M {
        input! {
            disk: usize,
        }
        if now == disk {
            continue;
        }
        for i in 0..N {
            if disk == ans[i] {
                ans[i] = now;
                now = disk;
            }
        }
    }
    for a in ans {
        println!("{}", a);
    }
}