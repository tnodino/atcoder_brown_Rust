// https://atcoder.jp/contests/arc118/tasks/arc118_a

use proconio::input;
use proconio::fastout;

fn f(x: usize, t: usize) -> usize {
    return (100 + t) * x / 100;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        t: usize,
        N: usize,
    }
    let mut ok: usize = 0;
    let mut ng: usize = N * 100;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        if f(mid, t)  - mid < N {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", f(ok, t) + 1);
}