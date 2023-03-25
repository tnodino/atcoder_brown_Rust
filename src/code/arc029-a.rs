// https://atcoder.jp/contests/arc029/tasks/arc029_1

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        t: [usize; N],
    }
    let su = t.iter().sum::<usize>();
    let mut ans = su;
    for bit in 0..1<<N {
        let mut time = 0;
        for i in 0..N {
            if bit & (1 << i) > 0 {
                time += t[i];
            }
        }
        ans = min(ans, max(time, su - time));
    }
    println!("{}", ans);
}