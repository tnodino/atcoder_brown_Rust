// https://atcoder.jp/contests/arc109/tasks/arc109_b

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let m = sqrt(n as f64) as usize * 2;
    let mut ok = 0;
    let mut ng = m + 1;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        if (mid + 1) * mid / 2 <= n + 1 {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", n - ok + 1);
}