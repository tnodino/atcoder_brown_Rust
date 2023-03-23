// https://atcoder.jp/contests/arc013/tasks/arc013_1

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        L: usize,
        P: usize,
        Q: usize,
        R: usize,
    }
    let arr1 = [N, M, L];
    let arr2 = [P, Q, R];
    let mut ans = 0;
    for comb in (0..3).permutations(3) {
        ans = max(ans, (arr1[0] / arr2[comb[0]]) * (arr1[1] / arr2[comb[1]]) * (arr1[2] / arr2[comb[2]]));
    }
    println!("{}", ans);
}