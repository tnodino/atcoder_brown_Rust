// https://atcoder.jp/contests/arc162/tasks/arc162_a

use proconio::input;

#[allow(non_snake_case)]
fn solve() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut ans = 0;
    'outer: for i in 0..N {
        for j in i+1..N {
            if P[i] > P[j] {
                continue 'outer;
            }
        }
        ans += 1;
    }
    println!("{}", ans);
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