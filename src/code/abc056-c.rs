// https://atcoder.jp/contests/abc056/tasks/arc070_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    let mut t = 0;
    for i in 1..=X {
        t += i;
        if X <= t {
            println!("{}", i);
            return;
        }
    }
}