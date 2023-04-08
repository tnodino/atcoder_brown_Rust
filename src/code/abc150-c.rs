// https://atcoder.jp/contests/abc150/tasks/abc150_c

use proconio::input;
use proconio::fastout;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
        Q: [usize; N],
    }
    let mut a = -1;
    let mut b = -1;
    for (i, perm) in (1..=N).permutations(N).enumerate() {
        if perm == P {
            a = i as isize;
        }
        if perm == Q {
            b = i as isize;
        }
    }
    println!("{}", (a - b).abs());
}