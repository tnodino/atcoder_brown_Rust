// https://atcoder.jp/contests/arc121/tasks/arc121_a

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vecx = Vec::new();
    let mut vecy = Vec::new();
    for i in 0..N {
        input! {
            x: isize,
            y: isize,
        }
        vecx.push((x, i));
        vecy.push((y, i));
    }
    vecx.sort();
    vecy.sort();
    let mut ans = Vec::new();
    for i in 0..2 {
        for j in 0..2 {
            let a = min(vecx[i].1, vecx[N-j-1].1);
            let b = max(vecx[i].1, vecx[N-j-1].1);
            ans.push(((vecx[i].0 - vecx[N-j-1].0).abs(), a, b));
            let a = min(vecy[i].1, vecy[N-j-1].1);
            let b = max(vecy[i].1, vecy[N-j-1].1);
            ans.push(((vecy[i].0 - vecy[N-j-1].0).abs(), a, b));
        }
    }
    ans.sort_by(|a, b| b.0.cmp(&a.0));
    for i in 1..ans.len() {
        if ans[0].1 != ans[i].1 || ans[0].2 != ans[i].2 {
            println!("{}", ans[i].0);
            return;
        }
    }
}