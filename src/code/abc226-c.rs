// https://atcoder.jp/contests/abc226/tasks/abc226_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cost: Vec<usize> = vec![0; N+1];
    let mut G = vec![Vec::new(); N+1];
    for i in 1..=N {
        input! {
            T: usize,
            K: usize,
            A: [usize; K],
        }
        cost[i] = T;
        for k in 0..K {
            G[i].push(A[k]);
        }
    }
    let mut flg = vec![false; N+1];
    flg[N] = true;
    let mut ans = 0;
    for i in (1..=N).rev() {
        if flg[i] {
            ans += cost[i];
            for k in 0..G[i].len() {
                flg[G[i][k]] = true;
            }
        }
    }
    println!("{}", ans);
}