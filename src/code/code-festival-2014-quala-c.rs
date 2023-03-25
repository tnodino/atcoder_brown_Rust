// https://atcoder.jp/contests/code-festival-2014-quala/tasks/code_festival_qualA_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let cnt1 = (A - 1) / 4 - (A - 1) / 100 + (A - 1) / 400;
    let cnt2 = B / 4 - B / 100 + B / 400;
    println!("{}", cnt2 - cnt1);
}